use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED079222ID");

#[program]
pub mod bump_can_079 {
    use super::*;
    pub fn set_value(ctx: Context<Set079>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set079<'info> {
    #[account(mut, seeds = [b"seed079", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc079>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc079 {
    pub value: u64,
}
