use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED040587ID");

#[program]
pub mod bump_can_040 {
    use super::*;
    pub fn set_value(ctx: Context<Set040>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set040<'info> {
    #[account(mut, seeds = [b"seed040", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc040>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc040 {
    pub value: u64,
}
