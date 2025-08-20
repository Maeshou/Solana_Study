use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED009290ID");

#[program]
pub mod bump_can_009 {
    use super::*;
    pub fn set_value(ctx: Context<Set009>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set009<'info> {
    #[account(mut, seeds = [b"seed009", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc009>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc009 {
    pub value: u64,
}
