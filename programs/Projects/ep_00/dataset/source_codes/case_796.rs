use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED096463ID");

#[program]
pub mod bump_can_096 {
    use super::*;
    pub fn set_value(ctx: Context<Set096>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set096<'info> {
    #[account(mut, seeds = [b"seed096", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc096>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc096 {
    pub value: u64,
}
