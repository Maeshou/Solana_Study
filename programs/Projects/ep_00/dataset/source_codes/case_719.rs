use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED019454ID");

#[program]
pub mod bump_can_019 {
    use super::*;
    pub fn set_value(ctx: Context<Set019>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set019<'info> {
    #[account(mut, seeds = [b"seed019", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc019>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc019 {
    pub value: u64,
}
