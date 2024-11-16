use protobuf::{Message, MessageField};
use async_nats::Client;
use crate::proto::account::Account;
use crate::proto::account_create::{CreateAccount, CreateAccountResponse};
use crate::proto::account_update::AccountUpdated;
use crate::proto::stats_get::{GetStats, GetStatsResponse};
use crate::store::Store;

#[tracing::instrument]
pub async fn create(db: Store, nc: Client, msg: async_nats::Message) -> anyhow::Result<()> {
    let request = CreateAccount::parse_from_bytes(&msg.payload)?;

    if let Some(reply) = msg.reply {

        // make sure we have at least one source for the user.
        if (request.discord_id.is_none()) {
            let mut resp = CreateAccountResponse::new();
            resp.success = false;
            resp.error = Some("Must have an account source.".to_string());
            let encoded: Vec<u8> = resp.write_to_bytes()?;
            nc.publish(reply, encoded.into()).await?;
            return Ok(());
        }

        // create account
        let mut account = Account::new();
        account.discord_id = request.discord_id;

        // save account
        let account = match db.create_account(&account).await {
            Ok(account) => account,
            Err(e) => {
                let mut resp = CreateAccountResponse::new();
                resp.success = false;
                resp.error = Some("Error creating account.".to_string());
                let encoded: Vec<u8> = resp.write_to_bytes()?;
                nc.publish(reply, encoded.into()).await?;
                return Ok(());

                tracing::error!("Error creating account: {:?}", e);
            }
        };

        // Build and Send Response
        let mut resp = CreateAccountResponse::new();
        resp.success = true;
        resp.account = MessageField::from(Some(account.clone()));
        let encoded: Vec<u8> = resp.write_to_bytes()?;
        nc.publish(reply, encoded.into()).await?;

        // Let's broadcast the account was created.
        let mut broadcast = AccountUpdated::new();
        broadcast.account = MessageField::some(account);
        let encoded: Vec<u8> = resp.write_to_bytes()?;
        nc.publish("accounts.updated", encoded.into()).await?;
    }

    Ok(())
}