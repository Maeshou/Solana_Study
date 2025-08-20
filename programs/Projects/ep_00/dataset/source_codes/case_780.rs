use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED080943ID");

#[program]
pub mod bump_can_080 {
    use super::*;
    pub fn set_value(ctx: Context<Set080>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set080<'info> {
    #[account(mut, seeds = [b"seed080", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc080>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc080 {
    pub value: u64,
}
