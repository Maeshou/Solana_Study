use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED004539ID");

#[program]
pub mod bump_can_004 {
    use super::*;
    pub fn set_value(ctx: Context<Set004>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set004<'info> {
    #[account(mut, seeds = [b"seed004", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc004>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc004 {
    pub value: u64,
}
