
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ApproveReportCtxipwx<'info> {
    #[account(mut)] pub approver: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_100 {
    use super::*;

    pub fn approve_report(ctx: Context<ApproveReportCtxipwx>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.approver;
        // custom logic for approve_report
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed approve_report logic");
        Ok(())
    }
}
