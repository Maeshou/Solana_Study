use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED084240ID");

#[program]
pub mod bump_can_084 {
    use super::*;
    pub fn set_value(ctx: Context<Set084>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set084<'info> {
    #[account(mut, seeds = [b"seed084", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc084>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc084 {
    pub value: u64,
}
