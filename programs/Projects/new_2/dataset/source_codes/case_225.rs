use anchor_lang::prelude::*;

declare_id!("VulnEx23000000000000000000000000000000000023");

#[program]
pub mod purchase_flow {
    pub fn purchase_item(ctx: Context<Ctx3>, qty: u64) -> Result<()> {
        // metrics_buf は未検証
        ctx.accounts.metrics_buf.data.borrow_mut()[0..8].copy_from_slice(&qty.to_le_bytes());
        // price_oracle は has_one で maintainer 検証済み
        let o = &mut ctx.accounts.price_oracle;
        o.last_price = o.last_price.saturating_sub(qty); 
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx3<'info> {
    #[account(mut, has_one = maintainer)]
    pub price_oracle: Account<'info, Oracle>,
    pub maintainer: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: メトリクスバッファ、所有者検証なし
    #[account(mut)]
    pub metrics_buf: AccountInfo<'info>,
    pub token_program: Program<'info, System>,
}
