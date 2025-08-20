use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED088138ID");

#[program]
pub mod bump_can_088 {
    use super::*;
    pub fn set_value(ctx: Context<Set088>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set088<'info> {
    #[account(mut, seeds = [b"seed088", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc088>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc088 {
    pub value: u64,
}
