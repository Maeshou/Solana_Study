use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED008591ID");

#[program]
pub mod bump_can_008 {
    use super::*;
    pub fn set_value(ctx: Context<Set008>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set008<'info> {
    #[account(mut, seeds = [b"seed008", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc008>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc008 {
    pub value: u64,
}
