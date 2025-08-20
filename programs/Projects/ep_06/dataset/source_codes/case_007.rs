use anchor_lang::prelude::*;
declare_id!("WITH0071111111111111111111111111111111111111");

#[program]
pub mod case007 {
    use super::*;
    pub fn execute_withdraw(ctx: Context<WithdrawContext>) -> Result<()> {
        // Withdraw from vault
        let mut vault = VaultAccount::try_from(&ctx.accounts.account_b.to_account_info())?;
        vault.balance = vault.balance.checked_sub(400).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawContext<'info> {
    /// CHECK: expecting WithdrawAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting WithdrawAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct WithdrawAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}