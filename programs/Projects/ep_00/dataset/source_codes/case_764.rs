use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED064810ID");

#[program]
pub mod bump_can_064 {
    use super::*;
    pub fn set_value(ctx: Context<Set064>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set064<'info> {
    #[account(mut, seeds = [b"seed064", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc064>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc064 {
    pub value: u64,
}
