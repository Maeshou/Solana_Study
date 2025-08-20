use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED036206ID");

#[program]
pub mod bump_can_036 {
    use super::*;
    pub fn set_value(ctx: Context<Set036>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set036<'info> {
    #[account(mut, seeds = [b"seed036", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc036>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc036 {
    pub value: u64,
}
