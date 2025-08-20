use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED074877ID");

#[program]
pub mod bump_can_074 {
    use super::*;
    pub fn set_value(ctx: Context<Set074>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set074<'info> {
    #[account(mut, seeds = [b"seed074", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc074>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc074 {
    pub value: u64,
}
