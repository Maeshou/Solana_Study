use anchor_lang::prelude::*;
declare_id!("Case0211111111111111111111111111111111111111");

#[program]
pub mod case021 {
    use super::*;
    pub fn execute_transfer_item(ctx: Context<TransferItemContext>) -> Result<()> {
        // Use Case 21: ゲーム内アイテム転送（TransferItem）
        // Vulnerable: using UncheckedAccount where TransferItemAccount is expected
        msg!("Executing execute_transfer_item for ゲーム内アイテム転送（TransferItem）");
        // Example logic (dummy operation)
        let mut acct_data = TransferItemAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferItemContext<'info> {
    /// CHECK: expecting TransferItemAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting TransferItemAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TransferItemAccount {
    pub dummy: u64,
}