use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED066308ID");

#[program]
pub mod bump_can_066 {
    use super::*;
    pub fn set_value(ctx: Context<Set066>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set066<'info> {
    #[account(mut, seeds = [b"seed066", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc066>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc066 {
    pub value: u64,
}
