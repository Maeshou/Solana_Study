use anchor_lang::prelude::*;
declare_id!("DISS0701111111111111111111111111111111111111");

#[program]
pub mod case070 {
    use super::*;
    pub fn execute_dissolvefund(ctx: Context<DissolveFundContext>) -> Result<()> {
        // Fund or airdrop logic
        let mut acct = AirdropAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct.claimed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DissolveFundContext<'info> {
    /// CHECK: expecting DissolveFundAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting DissolveFundAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DissolveFundAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}