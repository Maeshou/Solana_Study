use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED039913ID");

#[program]
pub mod bump_can_039 {
    use super::*;
    pub fn set_value(ctx: Context<Set039>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set039<'info> {
    #[account(mut, seeds = [b"seed039", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc039>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc039 {
    pub value: u64,
}
