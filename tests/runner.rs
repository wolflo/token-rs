use anyhow::Result;
use ethers::{
    signers::Signer,
    utils::{Ganache, GanacheInstance},
};

mod tests;
mod types;
mod utils;
use types::{Action, Ctx};
use utils::setup;

// Collect test functions, registered in tests.rs
inventory::collect!(Action<Ctx>);

#[tokio::main]
async fn main() -> Result<()> {
    let node: GanacheInstance = Ganache::new().spawn();
    let ctx = setup(&node, 3).await?;
    for act in inventory::iter::<Action<Ctx>> {
        // Need to generate a new snapshot every time
        let snap_id = ctx.client.snapshot().await?;
        act.apply(ctx.clone()).await?;
        ctx.client.revert_to_snapshot(snap_id).await?;
    }
    Ok(())
}
