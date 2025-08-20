use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED023681ID");

#[program]
pub mod bump_can_023 {
    use super::*;
    pub fn set_value(ctx: Context<Set023>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set023<'info> {
    #[account(mut, seeds = [b"seed023", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc023>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc023 {
    pub value: u64,
}
