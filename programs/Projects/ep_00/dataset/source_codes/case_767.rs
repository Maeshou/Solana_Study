use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED067447ID");

#[program]
pub mod bump_can_067 {
    use super::*;
    pub fn set_value(ctx: Context<Set067>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set067<'info> {
    #[account(mut, seeds = [b"seed067", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc067>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc067 {
    pub value: u64,
}
