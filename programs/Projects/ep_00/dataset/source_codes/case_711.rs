use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED011743ID");

#[program]
pub mod bump_can_011 {
    use super::*;
    pub fn set_value(ctx: Context<Set011>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set011<'info> {
    #[account(mut, seeds = [b"seed011", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc011>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc011 {
    pub value: u64,
}
