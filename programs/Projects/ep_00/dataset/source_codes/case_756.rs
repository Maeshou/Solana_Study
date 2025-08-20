use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED056676ID");

#[program]
pub mod bump_can_056 {
    use super::*;
    pub fn set_value(ctx: Context<Set056>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set056<'info> {
    #[account(mut, seeds = [b"seed056", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc056>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc056 {
    pub value: u64,
}
