use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED083480ID");

#[program]
pub mod bump_can_083 {
    use super::*;
    pub fn set_value(ctx: Context<Set083>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set083<'info> {
    #[account(mut, seeds = [b"seed083", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc083>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc083 {
    pub value: u64,
}
