use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED030920ID");

#[program]
pub mod bump_can_030 {
    use super::*;
    pub fn set_value(ctx: Context<Set030>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set030<'info> {
    #[account(mut, seeds = [b"seed030", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc030>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc030 {
    pub value: u64,
}
