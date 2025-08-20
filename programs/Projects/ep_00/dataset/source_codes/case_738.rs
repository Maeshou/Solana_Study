use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED038475ID");

#[program]
pub mod bump_can_038 {
    use super::*;
    pub fn set_value(ctx: Context<Set038>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set038<'info> {
    #[account(mut, seeds = [b"seed038", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc038>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc038 {
    pub value: u64,
}
