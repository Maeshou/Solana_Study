use anchor_lang::prelude::*;

declare_id!("OracleUpdV00000000000000000000000000000000");

#[program]
pub mod oracle_update_vuln {
    pub fn update_price(ctx: Context<Upd>, price: u64) -> Result<()> {
        // oracle.admin の検証なし
        let o = &mut ctx.accounts.oracle;
        o.price = price;
        o.update_count = o.update_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Upd<'info> {
    #[account(mut)]
    pub oracle: Account<'info, OracleData>,
}

#[account]
pub struct OracleData {
    pub admin: Pubkey,
    pub price: u64,
    pub update_count: u64,
}
