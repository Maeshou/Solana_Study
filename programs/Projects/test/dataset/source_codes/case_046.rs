use anchor_lang::prelude::*;
declare_id!("Case0461111111111111111111111111111111111111");

#[program]
pub mod case046 {
    use super::*;
    pub fn execute_join_guild(ctx: Context<JoinGuildContext>) -> Result<()> {
        // Use Case 46: ギルド加入申請（JoinGuild）
        // Vulnerable: using UncheckedAccount where JoinGuildAccount is expected
        msg!("Executing execute_join_guild for ギルド加入申請（JoinGuild）");
        // Example logic (dummy operation)
        let mut acct_data = JoinGuildAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct JoinGuildContext<'info> {
    /// CHECK: expecting JoinGuildAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting JoinGuildAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct JoinGuildAccount {
    pub dummy: u64,
}