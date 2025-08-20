use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED047871ID");

#[program]
pub mod bump_can_047 {
    use super::*;
    pub fn set_value(ctx: Context<Set047>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set047<'info> {
    #[account(mut, seeds = [b"seed047", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc047>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc047 {
    pub value: u64,
}
