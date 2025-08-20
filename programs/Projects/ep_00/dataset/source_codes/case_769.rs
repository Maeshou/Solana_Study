use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED069442ID");

#[program]
pub mod bump_can_069 {
    use super::*;
    pub fn set_value(ctx: Context<Set069>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set069<'info> {
    #[account(mut, seeds = [b"seed069", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc069>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc069 {
    pub value: u64,
}
