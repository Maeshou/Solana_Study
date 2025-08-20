use anchor_lang::prelude::*;
declare_id!("Case0441111111111111111111111111111111111111");

#[program]
pub mod case044 {
    use super::*;
    pub fn execute_craft_item(ctx: Context<CraftItemContext>) -> Result<()> {
        // Use Case 44: アイテムクラフト（CraftItem）
        // Vulnerable: using UncheckedAccount where CraftItemAccount is expected
        msg!("Executing execute_craft_item for アイテムクラフト（CraftItem）");
        // Example logic (dummy operation)
        let mut acct_data = CraftItemAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CraftItemContext<'info> {
    /// CHECK: expecting CraftItemAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting CraftItemAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CraftItemAccount {
    pub dummy: u64,
}