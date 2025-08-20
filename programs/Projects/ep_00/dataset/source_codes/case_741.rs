use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED041166ID");

#[program]
pub mod bump_can_041 {
    use super::*;
    pub fn set_value(ctx: Context<Set041>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set041<'info> {
    #[account(mut, seeds = [b"seed041", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc041>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc041 {
    pub value: u64,
}
