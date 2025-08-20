use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED081351ID");

#[program]
pub mod bump_can_081 {
    use super::*;
    pub fn set_value(ctx: Context<Set081>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set081<'info> {
    #[account(mut, seeds = [b"seed081", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc081>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc081 {
    pub value: u64,
}
