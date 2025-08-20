use anchor_lang::prelude::*;
declare_id!("CAST0141111111111111111111111111111111111111");

#[program]
pub mod case014 {
    use super::*;
    pub fn execute_castvote(ctx: Context<CastVoteContext>) -> Result<()> {
        // Governance action
        let mut gov = GovernanceAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        gov.votes = gov.votes.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CastVoteContext<'info> {
    /// CHECK: expecting CastVoteAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting CastVoteAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CastVoteAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}