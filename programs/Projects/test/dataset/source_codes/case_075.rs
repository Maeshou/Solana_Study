use anchor_lang::prelude::*;
declare_id!("Case0751111111111111111111111111111111111111");

#[program]
pub mod case075 {
    use super::*;
    pub fn execute_copyright_manage(ctx: Context<CopyrightManageContext>) -> Result<()> {
        // Use Case 75: 著作権管理（CopyrightManage）登録
        // Vulnerable: using UncheckedAccount where CopyrightManageAccount is expected
        msg!("Executing execute_copyright_manage for 著作権管理（CopyrightManage）登録");
        // Example logic (dummy operation)
        let mut acct_data = CopyrightManageAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CopyrightManageContext<'info> {
    /// CHECK: expecting CopyrightManageAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting CopyrightManageAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CopyrightManageAccount {
    pub dummy: u64,
}