
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimPremiumCtxeqix<'info> {
    #[account(mut)] pub option: Account<'info, DataAccount>,
    #[account(mut)] pub seller: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_093 {
    use super::*;

    pub fn claim_premium(ctx: Context<ClaimPremiumCtxeqix>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.option;
        // custom logic for claim_premium
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed claim_premium logic");
        Ok(())
    }
}
