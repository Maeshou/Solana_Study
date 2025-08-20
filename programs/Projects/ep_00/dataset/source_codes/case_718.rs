use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED018714ID");

#[program]
pub mod bump_can_018 {
    use super::*;
    pub fn set_value(ctx: Context<Set018>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set018<'info> {
    #[account(mut, seeds = [b"seed018", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc018>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc018 {
    pub value: u64,
}
