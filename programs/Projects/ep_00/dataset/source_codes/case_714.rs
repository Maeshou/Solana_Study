use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED014388ID");

#[program]
pub mod bump_can_014 {
    use super::*;
    pub fn set_value(ctx: Context<Set014>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set014<'info> {
    #[account(mut, seeds = [b"seed014", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc014>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc014 {
    pub value: u64,
}
