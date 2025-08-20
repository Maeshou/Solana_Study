use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED100964ID");

#[program]
pub mod bump_can_100 {
    use super::*;
    pub fn set_value(ctx: Context<Set100>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set100<'info> {
    #[account(mut, seeds = [b"seed100", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc100>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc100 {
    pub value: u64,
}
