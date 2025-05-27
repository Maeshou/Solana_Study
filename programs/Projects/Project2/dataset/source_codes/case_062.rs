
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ResetCtxbayx<'info> {
    #[account(mut)] pub counter: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_062 {
    use super::*;

    pub fn reset(ctx: Context<ResetCtxbayx>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.counter;
        // custom logic for reset
        assert!(ctx.accounts.counter.data > 0); acct.data -= amount;
        msg!("Executed reset logic");
        Ok(())
    }
}
