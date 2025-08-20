use anchor_lang::prelude::*;
declare_id!("DEPO0061111111111111111111111111111111111111");

#[program]
pub mod case006 {
    use super::*;
    pub fn execute_deposit(ctx: Context<DepositContext>) -> Result<()> {
        // Deposit to vault
        let mut vault = VaultAccount::try_from(&ctx.accounts.account_b.to_account_info())?;
        vault.balance = vault.balance.checked_add(1000).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositContext<'info> {
    /// CHECK: expecting DepositAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting DepositAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DepositAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}