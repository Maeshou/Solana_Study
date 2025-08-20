use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED075639ID");

#[program]
pub mod bump_can_075 {
    use super::*;
    pub fn set_value(ctx: Context<Set075>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set075<'info> {
    #[account(mut, seeds = [b"seed075", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc075>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc075 {
    pub value: u64,
}
