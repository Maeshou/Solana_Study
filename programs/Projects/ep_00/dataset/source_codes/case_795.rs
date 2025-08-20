use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED095628ID");

#[program]
pub mod bump_can_095 {
    use super::*;
    pub fn set_value(ctx: Context<Set095>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set095<'info> {
    #[account(mut, seeds = [b"seed095", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc095>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc095 {
    pub value: u64,
}
