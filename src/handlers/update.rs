use protobuf::{Message, MessageField};
use async_nats::Client;
use crate::proto::account_update::{AccountUpdated, UpdateAccount, UpdateAccountResponse};
use crate::store::Store;

#[tracing::instrument]
pub async fn update(db: Store, nc: Client, msg: async_nats::Message) -> anyhow::Result<()> {
    let request = UpdateAccount::parse_from_bytes(&msg.payload)?;

    if let Some(reply) = msg.reply {

        // make sure we have at least one source for the user.
        if request.account.discord_id.is_none() {
            let mut resp = UpdateAccountResponse::new();
            resp.success = false;
            resp.error = Some("Must have an account source.".to_string());
            let encoded: Vec<u8> = resp.write_to_bytes()?;
            nc.publish(reply, encoded.into()).await?;
            return Ok(());
        }

        // save account
        let account = match db.update_account(&request.account).await {
            Ok(account) => account,
            Err(e) => {
                tracing::error!("Error updating account: {:?}", e);

                let mut resp = UpdateAccountResponse::new();
                resp.success = false;
                resp.error = Some("Error creating account.".to_string());
                let encoded: Vec<u8> = resp.write_to_bytes()?;
                nc.publish(reply, encoded.into()).await?;
                return Ok(());
            }
        };

        // Build and Send Response
        let mut resp = UpdateAccountResponse::new();
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