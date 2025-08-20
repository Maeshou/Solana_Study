use anchor_lang::prelude::*;
declare_id!("Case0321111111111111111111111111111111111111");

#[program]
pub mod case032 {
    use super::*;
    pub fn execute_distributed_storage(ctx: Context<DistributedStorageContext>) -> Result<()> {
        // Use Case 32: 分散型保存（DistributedStorage）書き込み
        // Vulnerable: using UncheckedAccount where DistributedStorageAccount is expected
        msg!("Executing execute_distributed_storage for 分散型保存（DistributedStorage）書き込み");
        // Example logic (dummy operation)
        let mut acct_data = DistributedStorageAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DistributedStorageContext<'info> {
    /// CHECK: expecting DistributedStorageAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting DistributedStorageAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DistributedStorageAccount {
    pub dummy: u64,
}