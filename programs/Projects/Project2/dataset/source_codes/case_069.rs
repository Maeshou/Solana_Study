
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeTournamentCtxxsnq<'info> {
    #[account(mut)] pub tournament: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_069 {
    use super::*;

    pub fn initialize_tournament(ctx: Context<InitializeTournamentCtxxsnq>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.tournament;
        // custom logic for initialize_tournament
        assert!(ctx.accounts.tournament.data > 0); acct.data -= amount;
        msg!("Executed initialize_tournament logic");
        Ok(())
    }
}
