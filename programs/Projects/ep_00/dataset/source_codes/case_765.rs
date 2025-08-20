use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED065716ID");

#[program]
pub mod bump_can_065 {
    use super::*;
    pub fn set_value(ctx: Context<Set065>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set065<'info> {
    #[account(mut, seeds = [b"seed065", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc065>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc065 {
    pub value: u64,
}
