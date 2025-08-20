use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED035918ID");

#[program]
pub mod bump_can_035 {
    use super::*;
    pub fn set_value(ctx: Context<Set035>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set035<'info> {
    #[account(mut, seeds = [b"seed035", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc035>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc035 {
    pub value: u64,
}
