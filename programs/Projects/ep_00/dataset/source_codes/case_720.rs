use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED020311ID");

#[program]
pub mod bump_can_020 {
    use super::*;
    pub fn set_value(ctx: Context<Set020>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set020<'info> {
    #[account(mut, seeds = [b"seed020", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc020>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc020 {
    pub value: u64,
}
