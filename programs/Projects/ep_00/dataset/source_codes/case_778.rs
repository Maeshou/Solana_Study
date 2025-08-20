use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED078661ID");

#[program]
pub mod bump_can_078 {
    use super::*;
    pub fn set_value(ctx: Context<Set078>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set078<'info> {
    #[account(mut, seeds = [b"seed078", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc078>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc078 {
    pub value: u64,
}
