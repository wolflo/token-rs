use anyhow::Result;
use crate::*;
use ethers::{core::types::U256, utils::parse_ether};

// Must register all test functions
inventory::submit!(&test_name as Action<Ctx>);
inventory::submit!(&test_symbol as Action<Ctx>);
inventory::submit!(&test_mint as Action<Ctx>);
inventory::submit!(&test_transfer as Action<Ctx>);

pub async fn test_name(ctx: Ctx) -> Result<()> {
    println!("Testing name.");
    let name = &ctx.token.name().call().await?;
    assert_eq!(name, "Token");
    Ok(())
}

pub async fn test_symbol(ctx: Ctx) -> Result<()> {
    println!("Testing symbol.");
    let symbol = &ctx.token.symbol().call().await?;
    assert_eq!(symbol, "TOK");
    Ok(())
}

pub async fn test_mint(ctx: Ctx) -> Result<()> {
    println!("Testing mint.");
    let user = ctx.accts[1].address();
    let amt = U256::from(200);

    ctx.token.mint(user, amt).send().await?;

    let bal = ctx.token.balance_of(user).call().await?;
    assert_eq!(bal, amt);
    Ok(())
}

pub async fn test_transfer(ctx: Ctx) -> Result<()> {
    println!("Testing transfer.");

    let src = ctx.accts[1].address();
    let dst = ctx.accts[2].address();
    let mint_amt = parse_ether(100usize).unwrap();
    let send_amt = parse_ether(50usize).unwrap();
    let bal_src0 = ctx.token.balance_of(src).call().await?;
    let bal_dst0 = ctx.token.balance_of(dst).call().await?;

    // state properly reset between tests
    assert_eq!(bal_src0, U256::from(0usize));

    // mint tokens to sender
    ctx.token.mint(src, mint_amt).send().await?;

    // transfer tokens
    ctx.token.transfer(dst, send_amt).from(src).send().await?;

    let bal_src1 = ctx.token.balance_of(src).call().await?;
    let bal_dst1 = ctx.token.balance_of(dst).call().await?;
    assert_eq!(bal_dst1, bal_dst0 + send_amt);
    assert_eq!(bal_src1, mint_amt - send_amt);

    Ok(())
}
