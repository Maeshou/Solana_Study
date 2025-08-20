use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED077696ID");

#[program]
pub mod bump_can_077 {
    use super::*;
    pub fn set_value(ctx: Context<Set077>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set077<'info> {
    #[account(mut, seeds = [b"seed077", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc077>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc077 {
    pub value: u64,
}
