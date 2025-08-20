use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED024568ID");

#[program]
pub mod bump_can_024 {
    use super::*;
    pub fn set_value(ctx: Context<Set024>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set024<'info> {
    #[account(mut, seeds = [b"seed024", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc024>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc024 {
    pub value: u64,
}
