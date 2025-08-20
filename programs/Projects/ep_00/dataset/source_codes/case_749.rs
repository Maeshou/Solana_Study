use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED049405ID");

#[program]
pub mod bump_can_049 {
    use super::*;
    pub fn set_value(ctx: Context<Set049>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set049<'info> {
    #[account(mut, seeds = [b"seed049", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc049>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc049 {
    pub value: u64,
}
