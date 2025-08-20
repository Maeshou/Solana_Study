use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED012589ID");

#[program]
pub mod bump_can_012 {
    use super::*;
    pub fn set_value(ctx: Context<Set012>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set012<'info> {
    #[account(mut, seeds = [b"seed012", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc012>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc012 {
    pub value: u64,
}
