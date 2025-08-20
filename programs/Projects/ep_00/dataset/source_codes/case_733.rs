use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED033579ID");

#[program]
pub mod bump_can_033 {
    use super::*;
    pub fn set_value(ctx: Context<Set033>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set033<'info> {
    #[account(mut, seeds = [b"seed033", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc033>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc033 {
    pub value: u64,
}
