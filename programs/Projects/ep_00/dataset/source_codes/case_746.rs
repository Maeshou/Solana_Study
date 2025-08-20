use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED046871ID");

#[program]
pub mod bump_can_046 {
    use super::*;
    pub fn set_value(ctx: Context<Set046>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set046<'info> {
    #[account(mut, seeds = [b"seed046", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc046>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc046 {
    pub value: u64,
}
