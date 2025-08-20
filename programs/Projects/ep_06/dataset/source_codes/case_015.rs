use anchor_lang::prelude::*;
declare_id!("PROP0151111111111111111111111111111111111111");

#[program]
pub mod case015 {
    use super::*;
    pub fn execute_propose(ctx: Context<ProposeContext>) -> Result<()> {
        // Governance action
        let mut gov = GovernanceAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        gov.votes = gov.votes.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProposeContext<'info> {
    /// CHECK: expecting ProposeAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ProposeAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProposeAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}