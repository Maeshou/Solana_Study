use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED051802ID");

#[program]
pub mod bump_can_051 {
    use super::*;
    pub fn set_value(ctx: Context<Set051>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set051<'info> {
    #[account(mut, seeds = [b"seed051", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc051>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc051 {
    pub value: u64,
}
