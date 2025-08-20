use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED098627ID");

#[program]
pub mod bump_can_098 {
    use super::*;
    pub fn set_value(ctx: Context<Set098>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set098<'info> {
    #[account(mut, seeds = [b"seed098", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc098>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc098 {
    pub value: u64,
}
