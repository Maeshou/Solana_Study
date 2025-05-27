
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct LeaveTournamentCtxmsei<'info> {
    #[account(mut)] pub tournament: Account<'info, DataAccount>,
    #[account(mut)] pub player: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_071 {
    use super::*;

    pub fn leave_tournament(ctx: Context<LeaveTournamentCtxmsei>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.tournament;
        // custom logic for leave_tournament
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed leave_tournament logic");
        Ok(())
    }
}
