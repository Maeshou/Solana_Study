use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED093258ID");

#[program]
pub mod bump_can_093 {
    use super::*;
    pub fn set_value(ctx: Context<Set093>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set093<'info> {
    #[account(mut, seeds = [b"seed093", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc093>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc093 {
    pub value: u64,
}
