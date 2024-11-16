use protobuf::{Message, MessageField};
use async_nats::Client;
use crate::proto::account::Account;
use crate::proto::account_get::{GetAccount, GetAccountResponse};
use crate::store::Store;

#[tracing::instrument]
pub async fn get(db: Store, nc: Client, msg: async_nats::Message) -> anyhow::Result<()> {
    let request = GetAccount::parse_from_bytes(&msg.payload)?;

    if let Some(reply) = msg.reply {

        let mut account: Option<Account> = None;

        if request.account_id.is_some() {
            account = db.get_by_id(request.account_id.unwrap().parse::<i64>()?).await?;
        } else if request.discord_id.is_some() {
            account = db.get_by_discord(request.discord_id.unwrap().as_str()).await?;
        } else if request.minecraft_id.is_some() {
            // ignore the request we can't do anything.
            return Ok(());
        }

        // Build and Send Response
        let mut resp = GetAccountResponse::new();
        resp.account = MessageField::from(account);
        let encoded: Vec<u8> = resp.write_to_bytes()?;
        nc.publish(reply, encoded.into()).await?;
    }

    Ok(())
}