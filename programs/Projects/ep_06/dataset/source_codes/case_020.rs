use anchor_lang::prelude::*;
declare_id!("MULT0201111111111111111111111111111111111111");

#[program]
pub mod case020 {
    use super::*;
    pub fn execute_multisigexecute(ctx: Context<MultiSigExecuteContext>) -> Result<()> {
        // MultiSig execute
        let mut ms = MultiSigAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        ms.executed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MultiSigExecuteContext<'info> {
    /// CHECK: expecting MultiSigExecuteAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting MultiSigExecuteAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MultiSigExecuteAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}