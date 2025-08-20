use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED044439ID");

#[program]
pub mod bump_can_044 {
    use super::*;
    pub fn set_value(ctx: Context<Set044>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set044<'info> {
    #[account(mut, seeds = [b"seed044", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc044>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc044 {
    pub value: u64,
}
