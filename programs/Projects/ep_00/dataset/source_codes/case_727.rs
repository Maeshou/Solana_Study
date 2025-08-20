use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED027585ID");

#[program]
pub mod bump_can_027 {
    use super::*;
    pub fn set_value(ctx: Context<Set027>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set027<'info> {
    #[account(mut, seeds = [b"seed027", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc027>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc027 {
    pub value: u64,
}
