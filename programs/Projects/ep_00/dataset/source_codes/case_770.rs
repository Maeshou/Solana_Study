use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED070428ID");

#[program]
pub mod bump_can_070 {
    use super::*;
    pub fn set_value(ctx: Context<Set070>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set070<'info> {
    #[account(mut, seeds = [b"seed070", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc070>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc070 {
    pub value: u64,
}
