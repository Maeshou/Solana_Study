use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED037127ID");

#[program]
pub mod bump_can_037 {
    use super::*;
    pub fn set_value(ctx: Context<Set037>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set037<'info> {
    #[account(mut, seeds = [b"seed037", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc037>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc037 {
    pub value: u64,
}
