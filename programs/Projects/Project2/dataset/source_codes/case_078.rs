
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Swap3Ctxjdrp<'info> {
    #[account(mut)] pub pool3: Account<'info, DataAccount>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_078 {
    use super::*;

    pub fn swap3(ctx: Context<Swap3Ctxjdrp>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.pool3;
        // custom logic for swap3
        assert!(ctx.accounts.pool3.data > 0); acct.data -= amount;
        msg!("Executed swap3 logic");
        Ok(())
    }
}
