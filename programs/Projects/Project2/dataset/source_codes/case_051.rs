
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeGameCtxozdx<'info> {
    #[account(mut)] pub game: Account<'info, DataAccount>,
    #[account(mut)] pub creator: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_051 {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGameCtxozdx>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.game;
        // custom logic for initialize_game
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed initialize_game logic");
        Ok(())
    }
}
