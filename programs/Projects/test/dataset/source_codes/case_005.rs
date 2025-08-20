use anchor_lang::prelude::*;
declare_id!("Case0051111111111111111111111111111111111111");

#[program]
pub mod case005 {
    use super::*;
    pub fn execute_unstake(ctx: Context<UnstakeContext>) -> Result<()> {
        // Use Case 5: ステーキング解除（Unstake）
        // Vulnerable: using UncheckedAccount where UnstakeAccount is expected
        msg!("Executing execute_unstake for ステーキング解除（Unstake）");
        // Example logic (dummy operation)
        let mut acct_data = UnstakeAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UnstakeContext<'info> {
    /// CHECK: expecting UnstakeAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting UnstakeAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UnstakeAccount {
    pub dummy: u64,
}