use anchor_lang::prelude::*;
declare_id!("Case0471111111111111111111111111111111111111");

#[program]
pub mod case047 {
    use super::*;
    pub fn execute_leave_guild(ctx: Context<LeaveGuildContext>) -> Result<()> {
        // Use Case 47: ギルド脱退（LeaveGuild）
        // Vulnerable: using UncheckedAccount where LeaveGuildAccount is expected
        msg!("Executing execute_leave_guild for ギルド脱退（LeaveGuild）");
        // Example logic (dummy operation)
        let mut acct_data = LeaveGuildAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LeaveGuildContext<'info> {
    /// CHECK: expecting LeaveGuildAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting LeaveGuildAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LeaveGuildAccount {
    pub dummy: u64,
}