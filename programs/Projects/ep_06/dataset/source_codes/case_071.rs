use anchor_lang::prelude::*;
declare_id!("REGI0711111111111111111111111111111111111111");

#[program]
pub mod case071 {
    use super::*;
    pub fn execute_registerairdrop(ctx: Context<RegisterAirdropContext>) -> Result<()> {
        // Fund or airdrop logic
        let mut acct = AirdropAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct.claimed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterAirdropContext<'info> {
    /// CHECK: expecting RegisterAirdropAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RegisterAirdropAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RegisterAirdropAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}