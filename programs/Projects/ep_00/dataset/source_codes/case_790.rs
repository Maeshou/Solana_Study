use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED090145ID");

#[program]
pub mod bump_can_090 {
    use super::*;
    pub fn set_value(ctx: Context<Set090>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set090<'info> {
    #[account(mut, seeds = [b"seed090", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc090>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc090 {
    pub value: u64,
}
