use anchor_lang::prelude::*;

declare_id!("PoolFeeV2222222222222222222222222222222222");

#[program]
pub mod pool_fee_vuln {
    pub fn set_fee(ctx: Context<SetFee>, new_fee_bps: u16) -> Result<()> {
        // pool.owner の検証がない
        let pool = &mut ctx.accounts.pool;
        pool.fee_bps = new_fee_bps;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetFee<'info> {
    #[account(mut)]
    pub pool: Account<'info, FeePool>,
}

#[account]
pub struct FeePool {
    pub owner: Pubkey,
    pub fee_bps: u16,
}
