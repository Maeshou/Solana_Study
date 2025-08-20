use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED068622ID");

#[program]
pub mod bump_can_068 {
    use super::*;
    pub fn set_value(ctx: Context<Set068>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set068<'info> {
    #[account(mut, seeds = [b"seed068", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc068>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc068 {
    pub value: u64,
}
