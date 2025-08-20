use anchor_lang::prelude::*;
declare_id!("Case1001111111111111111111111111111111111111");

#[program]
pub mod case100 {
    use super::*;
    pub fn execute_amm(ctx: Context<AMMContext>) -> Result<()> {
        // Use Case 100: 分散型自動マーケットメーカー（AMM）スワップ実行
        // Vulnerable: using UncheckedAccount where AMMAccount is expected
        msg!("Executing execute_amm for 分散型自動マーケットメーカー（AMM）スワップ実行");
        // Example logic (dummy operation)
        let mut acct_data = AMMAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AMMContext<'info> {
    /// CHECK: expecting AMMAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting AMMAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AMMAccount {
    pub dummy: u64,
}