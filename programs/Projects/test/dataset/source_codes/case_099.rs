use anchor_lang::prelude::*;
declare_id!("Case0991111111111111111111111111111111111111");

#[program]
pub mod case099 {
    use super::*;
    pub fn execute_amm(ctx: Context<AMMContext>) -> Result<()> {
        // Use Case 99: 分散型自動マーケットメーカー（AMM）価格更新
        // Vulnerable: using UncheckedAccount where AMMAccount is expected
        msg!("Executing execute_amm for 分散型自動マーケットメーカー（AMM）価格更新");
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