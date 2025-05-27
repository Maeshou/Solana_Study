
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ResumeCtxnaci<'info> {
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_098 {
    use super::*;

    pub fn resume(ctx: Context<ResumeCtxnaci>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.authority;
        // custom logic for resume
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed resume logic");
        Ok(())
    }
}
