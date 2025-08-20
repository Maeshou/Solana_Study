use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED034776ID");

#[program]
pub mod bump_can_034 {
    use super::*;
    pub fn set_value(ctx: Context<Set034>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set034<'info> {
    #[account(mut, seeds = [b"seed034", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc034>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc034 {
    pub value: u64,
}
