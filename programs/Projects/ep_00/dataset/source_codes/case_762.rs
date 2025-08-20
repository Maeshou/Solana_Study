use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED062174ID");

#[program]
pub mod bump_can_062 {
    use super::*;
    pub fn set_value(ctx: Context<Set062>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set062<'info> {
    #[account(mut, seeds = [b"seed062", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc062>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc062 {
    pub value: u64,
}
