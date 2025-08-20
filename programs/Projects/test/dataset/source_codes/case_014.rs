use anchor_lang::prelude::*;
declare_id!("Case0141111111111111111111111111111111111111");

#[program]
pub mod case014 {
    use super::*;
    pub fn execute_cast_vote(ctx: Context<CastVoteContext>) -> Result<()> {
        // Use Case 14: ガバナンス投票（CastVote）
        // Vulnerable: using UncheckedAccount where CastVoteAccount is expected
        msg!("Executing execute_cast_vote for ガバナンス投票（CastVote）");
        // Example logic (dummy operation)
        let mut acct_data = CastVoteAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
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
}