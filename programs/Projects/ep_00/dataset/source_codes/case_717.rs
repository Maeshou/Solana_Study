use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED017770ID");

#[program]
pub mod bump_can_017 {
    use super::*;
    pub fn set_value(ctx: Context<Set017>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set017<'info> {
    #[account(mut, seeds = [b"seed017", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc017>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc017 {
    pub value: u64,
}
