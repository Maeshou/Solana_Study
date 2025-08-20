use anchor_lang::prelude::*;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpB3C2D1E0F9G8H7J6K5L4M3N2O1");

#[program]
pub mod tournament_registration {
    use super::*;

    /// トーナメントアカウントを初期化
    pub fn initialize_tournament(
        ctx: Context<InitializeTournament>,
        bump: u8,
        name: String,
    ) -> ProgramResult {
        require!(name.len() <= 50, ErrorCode::NameTooLong);
        let tour = &mut ctx.accounts.tournament;
        tour.owner = *ctx.accounts.admin.key;
        tour.bump = bump;
        tour.name = name;
        tour.teams = Vec::new();
        Ok(())
    }

    /// チームを登録
    pub fn register_team(
        ctx: Context<ModifyTournament>,
        team: Pubkey,
    ) -> ProgramResult {
        let tour = &mut ctx.accounts.tournament;
        require!(!tour.teams.contains(&team), ErrorCode::AlreadyRegistered);
        tour.teams.push(team);
        Ok(())
    }

    /// チームを登録解除
    pub fn unregister_team(
        ctx: Context<ModifyTournament>,
        team: Pubkey,
    ) -> ProgramResult {
        let tour = &mut ctx.accounts.tournament;
        require!(tour.teams.contains(&team), ErrorCode::NotRegistered);
        tour.teams.retain(|&t| t != team);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, name: String)]
pub struct InitializeTournament<'info> {
    #[account(
        init,
        seeds = [b"tournament", admin.key().as_ref()],
        bump = bump,
        payer = admin,
        space = 8 + 32 + 1 + 4 + 50 + 4 + 32 * 100,
    )]
    pub tournament: Account<'info, Tournament>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ModifyTournament<'info> {
    #[account(
        mut,
        seeds = [b"tournament", tournament.owner.as_ref()],
        bump = tournament.bump,
        has_one = owner,
    )]
    pub tournament: Account<'info, Tournament>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Tournament {
    pub owner: Pubkey,
    pub bump: u8,
    pub name: String,
    pub teams: Vec<Pubkey>,
}

#[error]
pub enum ErrorCode {
    #[msg("Tournament name must be 50 characters or fewer.")]
    NameTooLong,
    #[msg("Team is already registered.")]
    AlreadyRegistered,
    #[msg("Team is not registered.")]
    NotRegistered,
}
