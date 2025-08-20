use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED061385ID");

#[program]
pub mod bump_can_061 {
    use super::*;
    pub fn set_value(ctx: Context<Set061>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set061<'info> {
    #[account(mut, seeds = [b"seed061", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc061>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc061 {
    pub value: u64,
}
