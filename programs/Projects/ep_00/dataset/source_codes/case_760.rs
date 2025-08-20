use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED060867ID");

#[program]
pub mod bump_can_060 {
    use super::*;
    pub fn set_value(ctx: Context<Set060>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set060<'info> {
    #[account(mut, seeds = [b"seed060", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc060>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc060 {
    pub value: u64,
}
