use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED086129ID");

#[program]
pub mod bump_can_086 {
    use super::*;
    pub fn set_value(ctx: Context<Set086>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set086<'info> {
    #[account(mut, seeds = [b"seed086", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc086>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc086 {
    pub value: u64,
}
