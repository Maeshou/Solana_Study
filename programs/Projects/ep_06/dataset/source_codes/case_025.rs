use anchor_lang::prelude::*;
declare_id!("SUBM0251111111111111111111111111111111111111");

#[program]
pub mod case025 {
    use super::*;
    pub fn execute_submitclaim(ctx: Context<SubmitClaimContext>) -> Result<()> {
        // Insurance claim logic
        let mut claim = ClaimAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        claim.status = 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitClaimContext<'info> {
    /// CHECK: expecting SubmitClaimAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SubmitClaimAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SubmitClaimAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}