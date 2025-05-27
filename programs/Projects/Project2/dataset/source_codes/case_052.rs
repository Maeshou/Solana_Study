
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct JoinGameCtxizgy<'info> {
    #[account(mut)] pub game: Account<'info, DataAccount>,
    #[account(mut)] pub player: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_052 {
    use super::*;

    pub fn join_game(ctx: Context<JoinGameCtxizgy>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.game;
        // custom logic for join_game
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed join_game logic");
        Ok(())
    }
}
