use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED028775ID");

#[program]
pub mod bump_can_028 {
    use super::*;
    pub fn set_value(ctx: Context<Set028>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set028<'info> {
    #[account(mut, seeds = [b"seed028", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc028>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc028 {
    pub value: u64,
}
