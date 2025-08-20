use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED050797ID");

#[program]
pub mod bump_can_050 {
    use super::*;
    pub fn set_value(ctx: Context<Set050>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set050<'info> {
    #[account(mut, seeds = [b"seed050", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc050>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc050 {
    pub value: u64,
}
