use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED057299ID");

#[program]
pub mod bump_can_057 {
    use super::*;
    pub fn set_value(ctx: Context<Set057>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set057<'info> {
    #[account(mut, seeds = [b"seed057", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc057>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc057 {
    pub value: u64,
}
