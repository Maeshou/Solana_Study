use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED016478ID");

#[program]
pub mod bump_can_016 {
    use super::*;
    pub fn set_value(ctx: Context<Set016>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set016<'info> {
    #[account(mut, seeds = [b"seed016", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc016>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc016 {
    pub value: u64,
}
