
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct JoinTournamentCtxcffu<'info> {
    #[account(mut)] pub tournament: Account<'info, DataAccount>,
    #[account(mut)] pub player: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_070 {
    use super::*;

    pub fn join_tournament(ctx: Context<JoinTournamentCtxcffu>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.tournament;
        // custom logic for join_tournament
        **ctx.accounts.tournament.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed join_tournament logic");
        Ok(())
    }
}
