use anchor_lang::prelude::*;
declare_id!("Case0221111111111111111111111111111111111111");

#[program]
pub mod case022 {
    use super::*;
    pub fn execute_sell_item(ctx: Context<SellItemContext>) -> Result<()> {
        // Use Case 22: ゲーム内アイテム販売（SellItem）
        // Vulnerable: using UncheckedAccount where SellItemAccount is expected
        msg!("Executing execute_sell_item for ゲーム内アイテム販売（SellItem）");
        // Example logic (dummy operation)
        let mut acct_data = SellItemAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SellItemContext<'info> {
    /// CHECK: expecting SellItemAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SellItemAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SellItemAccount {
    pub dummy: u64,
}