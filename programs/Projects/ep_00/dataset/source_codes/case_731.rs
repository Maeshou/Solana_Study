use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED031914ID");

#[program]
pub mod bump_can_031 {
    use super::*;
    pub fn set_value(ctx: Context<Set031>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set031<'info> {
    #[account(mut, seeds = [b"seed031", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc031>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc031 {
    pub value: u64,
}
