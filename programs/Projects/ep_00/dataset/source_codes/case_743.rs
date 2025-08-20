use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED043702ID");

#[program]
pub mod bump_can_043 {
    use super::*;
    pub fn set_value(ctx: Context<Set043>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set043<'info> {
    #[account(mut, seeds = [b"seed043", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc043>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc043 {
    pub value: u64,
}
