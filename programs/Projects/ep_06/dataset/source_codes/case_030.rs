use anchor_lang::prelude::*;
declare_id!("REGI0301111111111111111111111111111111111111");

#[program]
pub mod case030 {
    use super::*;
    pub fn execute_registerdid(ctx: Context<RegisterDIDContext>) -> Result<()> {
        // DID registry logic
        let mut did = DIDAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        did.registered = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterDIDContext<'info> {
    /// CHECK: expecting RegisterDIDAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RegisterDIDAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RegisterDIDAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}