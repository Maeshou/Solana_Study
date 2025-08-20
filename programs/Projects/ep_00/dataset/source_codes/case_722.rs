use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED022631ID");

#[program]
pub mod bump_can_022 {
    use super::*;
    pub fn set_value(ctx: Context<Set022>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set022<'info> {
    #[account(mut, seeds = [b"seed022", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc022>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc022 {
    pub value: u64,
}
