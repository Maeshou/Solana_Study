use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED092782ID");

#[program]
pub mod bump_can_092 {
    use super::*;
    pub fn set_value(ctx: Context<Set092>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set092<'info> {
    #[account(mut, seeds = [b"seed092", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc092>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc092 {
    pub value: u64,
}
