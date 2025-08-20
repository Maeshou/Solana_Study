use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED005962ID");

#[program]
pub mod bump_can_005 {
    use super::*;
    pub fn set_value(ctx: Context<Set005>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set005<'info> {
    #[account(mut, seeds = [b"seed005", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc005>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc005 {
    pub value: u64,
}
