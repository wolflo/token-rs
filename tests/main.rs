use anyhow::Result;
use std::panic::AssertUnwindSafe;
use futures::FutureExt;

use ethers::utils::{launch_ganache, Ganache, GanacheInstance};

mod types;
mod utils;
mod erc20_mintable_pausable;
use types::*;
use utils::setup;
use erc20_mintable_pausable::*;

#[tokio::main]
async fn main() -> Result<()> {
    let node: &GanacheInstance = &launch_ganache(Ganache::new()).await;
    let ctx = setup(&node, 3).await?;

    let tests: Vec<Box<dyn Runner>> = vec![
        Box::new(test_name),
        Box::new(test_symbol),
        Box::new(test_mint),
        Box::new(test_transfer),
    ];

    for (i, test) in tests.iter().enumerate() {
        let res = AssertUnwindSafe(
            test.run(ctx.clone()), //TODO: using &ctx here results in https://github.com/rust-lang/rust/issues/64650
        )
        .catch_unwind()
        .await.unwrap();

        match res {
            Ok(_) => println!("test {} passed.", i+1),
            Err(e) => println!("test {} failed! Error: {:?}", i+1, e)
        }
    }

    Ok(())
}
