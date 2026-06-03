use crate::{
    log::{init_log, read_from_log},
    server::serve,
    store::init_store,
};

use std::{path::PathBuf, sync::Arc};

mod log;
mod models;
mod server;
mod store;

fn main() -> anyhow::Result<()> {
    let lp = PathBuf::from("wal.log");
    let store = init_store();
    let log = init_log(lp.clone())?;
    read_from_log(&lp, Arc::clone(&store))?;
    serve(Arc::clone(&store), Arc::clone(&log))?;

    Ok(())
}
