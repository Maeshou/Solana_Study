use anchor_lang::prelude::*;
declare_id!("APPR0261111111111111111111111111111111111111");

#[program]
pub mod case026 {
    use super::*;
    pub fn execute_approveclaim(ctx: Context<ApproveClaimContext>) -> Result<()> {
        // Insurance claim logic
        let mut claim = ClaimAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        claim.status = 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ApproveClaimContext<'info> {
    /// CHECK: expecting ApproveClaimAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ApproveClaimAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ApproveClaimAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}