use anchor_lang::prelude::*;
declare_id!("Case0181111111111111111111111111111111111111");

#[program]
pub mod case018 {
    use super::*;
    pub fn execute_read_oracle(ctx: Context<ReadOracleContext>) -> Result<()> {
        // Use Case 18: オラクル参照（ReadOracle）
        // Vulnerable: using UncheckedAccount where ReadOracleAccount is expected
        msg!("Executing execute_read_oracle for オラクル参照（ReadOracle）");
        // Example logic (dummy operation)
        let mut acct_data = ReadOracleAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReadOracleContext<'info> {
    /// CHECK: expecting ReadOracleAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ReadOracleAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ReadOracleAccount {
    pub dummy: u64,
}