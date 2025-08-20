use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED048396ID");

#[program]
pub mod bump_can_048 {
    use super::*;
    pub fn set_value(ctx: Context<Set048>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set048<'info> {
    #[account(mut, seeds = [b"seed048", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc048>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc048 {
    pub value: u64,
}
