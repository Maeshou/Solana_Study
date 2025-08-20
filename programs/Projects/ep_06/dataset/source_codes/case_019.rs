use anchor_lang::prelude::*;
declare_id!("MULT0191111111111111111111111111111111111111");

#[program]
pub mod case019 {
    use super::*;
    pub fn execute_multisigcreate(ctx: Context<MultiSigCreateContext>) -> Result<()> {
        // MultiSig execute
        let mut ms = MultiSigAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        ms.executed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MultiSigCreateContext<'info> {
    /// CHECK: expecting MultiSigCreateAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting MultiSigCreateAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MultiSigCreateAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}