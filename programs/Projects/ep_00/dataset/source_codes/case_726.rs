use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED026784ID");

#[program]
pub mod bump_can_026 {
    use super::*;
    pub fn set_value(ctx: Context<Set026>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set026<'info> {
    #[account(mut, seeds = [b"seed026", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc026>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc026 {
    pub value: u64,
}
