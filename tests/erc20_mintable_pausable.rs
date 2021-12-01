use anyhow::Result;

// use crate::types::*;
use crate::*;
// use ethers::{prelude::*, core::types::U256, utils::parse_ether};
use ethers::{core::types::U256, utils::parse_ether};

pub async fn test_name(ctx: Context) -> Result<()> {
    println!("Testing name.");
    let name = &ctx.token.name().call().await?;
    assert_eq!(name, "Token");
    Ok(())
}

pub async fn test_symbol(ctx: Context) -> Result<()> {
    println!("Testing symbol.");
    let symbol = &ctx.token.symbol().call().await?;
    assert_eq!(symbol, "TOK");
    Ok(())
}

pub async fn test_mint(ctx: Context) -> Result<()> {
    println!("Testing mint.");
    let user = ctx.accts[0].address();
    let amt = U256::from(200);

    ctx.token.mint(user, amt).send().await?;

    let bal = ctx.token.balance_of(user).call().await?;
    assert_eq!(bal, amt);
    Ok(())
}

pub async fn test_transfer(ctx: Context) -> Result<()> {
    println!("Testing transfer.");

    let src = ctx.accts[1].address();
    let dst = ctx.accts[2].address();
    let mint_amt = parse_ether(100usize).unwrap();
    let send_amt = parse_ether(50usize).unwrap();

    ctx.token.mint(src, mint_amt).send().await?;
    ctx.token.transfer(dst, send_amt).from(src).send().await?;

    let bal_src = ctx.token.balance_of(src).call().await?;
    let bal_dst = ctx.token.balance_of(dst).call().await?;
    assert_eq!(bal_dst, send_amt);
    assert_eq!(bal_src, mint_amt - send_amt);

    Ok(())
}
