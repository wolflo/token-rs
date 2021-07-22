use anyhow::Result;

use crate::types::*;

pub async fn test_name(ctx: Context) -> Result<()> {
    let name = &ctx.token.name().call().await?;
    assert_eq!(name, "Token");
    Ok(())
}

pub async fn test_symbol(ctx: Context) -> Result<()> {
    let symbol = &ctx.token.symbol().call().await?;
    assert_eq!(symbol, "TOK");
    Ok(())
}

pub async fn test_transfer(ctx: Context) -> Result<()> {
    panic!("no tokens minted!");
}
