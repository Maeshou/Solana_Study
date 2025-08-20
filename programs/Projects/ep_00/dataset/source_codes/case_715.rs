use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED015705ID");

#[program]
pub mod bump_can_015 {
    use super::*;
    pub fn set_value(ctx: Context<Set015>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set015<'info> {
    #[account(mut, seeds = [b"seed015", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc015>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc015 {
    pub value: u64,
}
