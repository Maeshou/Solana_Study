use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED003405ID");

#[program]
pub mod bump_can_003 {
    use super::*;
    pub fn set_value(ctx: Context<Set003>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set003<'info> {
    #[account(mut, seeds = [b"seed003", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc003>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc003 {
    pub value: u64,
}
