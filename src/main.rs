use crate::{server::serve, store::init_store};

use std::sync::Arc;

mod models;
mod server;
mod store;

fn main() -> anyhow::Result<()> {
    let store = init_store();
    serve(Arc::clone(&store))?;

    Ok(())
}
