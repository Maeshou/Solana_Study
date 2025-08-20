use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED094420ID");

#[program]
pub mod bump_can_094 {
    use super::*;
    pub fn set_value(ctx: Context<Set094>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set094<'info> {
    #[account(mut, seeds = [b"seed094", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc094>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc094 {
    pub value: u64,
}
