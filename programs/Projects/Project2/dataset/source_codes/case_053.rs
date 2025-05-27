
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct LeaveGameCtxduxi<'info> {
    #[account(mut)] pub game: Account<'info, DataAccount>,
    #[account(mut)] pub player: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_053 {
    use super::*;

    pub fn leave_game(ctx: Context<LeaveGameCtxduxi>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.game;
        // custom logic for leave_game
        assert!(ctx.accounts.game.data > 0); acct.data -= amount;
        msg!("Executed leave_game logic");
        Ok(())
    }
}
