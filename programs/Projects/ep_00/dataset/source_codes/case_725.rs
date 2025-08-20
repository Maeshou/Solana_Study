use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED025923ID");

#[program]
pub mod bump_can_025 {
    use super::*;
    pub fn set_value(ctx: Context<Set025>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set025<'info> {
    #[account(mut, seeds = [b"seed025", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc025>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc025 {
    pub value: u64,
}
