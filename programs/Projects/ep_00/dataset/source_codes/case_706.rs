use anchor_lang::prelude::*;

declare_id!("9_BUMP_SEED006799ID");

#[program]
pub mod bump_can_006 {
    use super::*;
    pub fn set_value(ctx: Context<Set006>, val: u64) -> Result<()> {
        ctx.accounts.data_acc.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set006<'info> {
    #[account(mut, seeds = [b"seed006", user.key().as_ref()], bump)]
    pub data_acc: Account<'info, DataAcc006>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAcc006 {
    pub value: u64,
}
