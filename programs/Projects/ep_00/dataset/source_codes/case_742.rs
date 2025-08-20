use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED042243ID");

#[program]
pub mod bump_can_042 {
    use super::*;
    pub fn set_value(ctx: Context<Set042>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set042<'info> {
    #[account(mut, seeds = [b"seed042", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc042>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc042 {
    pub value: u64,
}
