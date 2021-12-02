use anyhow::Result;
use ethers::{
    utils::{GanacheInstance, Ganache},
    signers::Signer,
};

mod types;
mod utils;
mod tests;
use utils::setup;
use types::{Ctx, Action};

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
