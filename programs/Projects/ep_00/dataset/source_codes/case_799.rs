use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED099170ID");

#[program]
pub mod bump_can_099 {
    use super::*;
    pub fn set_value(ctx: Context<Set099>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set099<'info> {
    #[account(mut, seeds = [b"seed099", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc099>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc099 {
    pub value: u64,
}
