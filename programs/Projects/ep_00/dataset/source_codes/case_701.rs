use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED001591ID");

#[program]
pub mod bump_can_001 {
    use super::*;
    pub fn set_value(ctx: Context<Set001>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set001<'info> {
    #[account(mut, seeds = [b"seed001", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc001>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc001 {
    pub value: u64,
}
