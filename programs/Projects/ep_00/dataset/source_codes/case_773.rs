use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED073186ID");

#[program]
pub mod bump_can_073 {
    use super::*;
    pub fn set_value(ctx: Context<Set073>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set073<'info> {
    #[account(mut, seeds = [b"seed073", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc073>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc073 {
    pub value: u64,
}
