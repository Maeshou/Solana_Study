use anchor_lang::prelude::*;
declare_id!("Case0151111111111111111111111111111111111111");

#[program]
pub mod case015 {
    use super::*;
    pub fn execute_propose(ctx: Context<ProposeContext>) -> Result<()> {
        // Use Case 15: ガバナンス新規提案（Propose）
        // Vulnerable: using UncheckedAccount where ProposeAccount is expected
        msg!("Executing execute_propose for ガバナンス新規提案（Propose）");
        // Example logic (dummy operation)
        let mut acct_data = ProposeAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
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
}