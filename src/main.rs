mod proto;
mod util;
mod store;
mod handlers;

use anyhow::Result;
use tokio::task::JoinSet;
use crate::handlers::create::create;
use crate::handlers::get::get;
use crate::store::Store;

#[tokio::main]
async fn main() -> Result<()> {

    // get the app name, used for group and such
    let app_name = match util::get_app_name() {
        Some(name) => name,
        None => { return Err(anyhow::anyhow!("Could not  determine application name.")); },
    };

    // Setup logging
    util::setup_logging(app_name.as_str());

    // connect to db
    let db = util::connect_to_database().await?;
    let store = Store::new(db.clone());

    // connect to nats
    let nc = util::connect_to_nats().await?;

    let mut set = JoinSet::new();

    let _nc = nc.clone();
    let _store = store.clone();
    set.spawn(async move {
        util::handle_requests(_nc, "accounts.create", move|_nc, msg| {
            create(_store.clone(), _nc, msg)
        }).await.expect("accounts.create");
    });

    let _nc = nc.clone();
    let _store = store.clone();
    set.spawn(async move {
        util::handle_requests(_nc, "accounts.get", move|_nc, msg| {
            get(_store.clone(), _nc, msg)
        }).await.expect("accounts.get");
    });

    set.join_all().await;
    Ok(())
}
