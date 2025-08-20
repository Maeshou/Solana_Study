use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED059933ID");

#[program]
pub mod bump_can_059 {
    use super::*;
    pub fn set_value(ctx: Context<Set059>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set059<'info> {
    #[account(mut, seeds = [b"seed059", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc059>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc059 {
    pub value: u64,
}
