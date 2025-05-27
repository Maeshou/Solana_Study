
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DrawWinnerCtxixia<'info> {
    #[account(mut)] pub lottery: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_064 {
    use super::*;

    pub fn draw_winner(ctx: Context<DrawWinnerCtxixia>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.lottery;
        // custom logic for draw_winner
        assert!(ctx.accounts.lottery.data > 0); acct.data -= amount;
        msg!("Executed draw_winner logic");
        Ok(())
    }
}
