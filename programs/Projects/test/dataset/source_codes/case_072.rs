use anchor_lang::prelude::*;
declare_id!("Case0721111111111111111111111111111111111111");

#[program]
pub mod case072 {
    use super::*;
    pub fn execute_claim_airdrop(ctx: Context<ClaimAirdropContext>) -> Result<()> {
        // Use Case 72: エアドロップ請求（ClaimAirdrop）
        // Vulnerable: using UncheckedAccount where ClaimAirdropAccount is expected
        msg!("Executing execute_claim_airdrop for エアドロップ請求（ClaimAirdrop）");
        // Example logic (dummy operation)
        let mut acct_data = ClaimAirdropAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimAirdropContext<'info> {
    /// CHECK: expecting ClaimAirdropAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ClaimAirdropAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ClaimAirdropAccount {
    pub dummy: u64,
}