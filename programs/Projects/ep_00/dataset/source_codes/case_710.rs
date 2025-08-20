use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED010510ID");

#[program]
pub mod bump_can_010 {
    use super::*;
    pub fn set_value(ctx: Context<Set010>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set010<'info> {
    #[account(mut, seeds = [b"seed010", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc010>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc010 {
    pub value: u64,
}
