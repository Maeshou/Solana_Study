use anchor_lang::prelude::*;
declare_id!("Case0541111111111111111111111111111111111111");

#[program]
pub mod case054 {
    use super::*;
    pub fn execute_swap(ctx: Context<SwapContext>) -> Result<()> {
        // Use Case 54: トークン転換（Swap）
        // Vulnerable: using UncheckedAccount where SwapAccount is expected
        msg!("Executing execute_swap for トークン転換（Swap）");
        // Example logic (dummy operation)
        let mut acct_data = SwapAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SwapContext<'info> {
    /// CHECK: expecting SwapAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SwapAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SwapAccount {
    pub dummy: u64,
}