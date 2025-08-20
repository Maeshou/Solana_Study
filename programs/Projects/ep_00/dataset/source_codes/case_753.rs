use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED053438ID");

#[program]
pub mod bump_can_053 {
    use super::*;
    pub fn set_value(ctx: Context<Set053>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set053<'info> {
    #[account(mut, seeds = [b"seed053", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc053>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc053 {
    pub value: u64,
}
