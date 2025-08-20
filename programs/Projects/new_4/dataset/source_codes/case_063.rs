// 3. トーナメント登録＋参加ログ
use anchor_lang::prelude::*;
declare_id!("NFVG3Tourna1111222233334444CCCC5555");

#[program]
pub mod misinit_tournament_v8 {
    use super::*;

    pub fn init_tournament(
        ctx: Context<InitTournament>,
        capacity: u16,
    ) -> Result<()> {
        let t = &mut ctx.accounts.tournament;
        t.capacity = capacity;
        t.registered = 0;
        Ok(())
    }

    pub fn register_player(
        ctx: Context<InitTournament>,
    ) -> Result<()> {
        let t = &mut ctx.accounts.tournament;
        require!(t.registered < t.capacity, ErrorCode2::Full);
        t.registered = t.registered.checked_add(1).unwrap();
        Ok(())
    }

    pub fn log_registration(
        ctx: Context<InitTournament>,
        player: Pubkey,
    ) -> Result<()> {
        let log = &mut ctx.accounts.reg_log;
        log.players.push(player);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTournament<'info> {
    #[account(init, payer = organizer, space = 8 + 2 + 2)]
    pub tournament: Account<'info, TournamentData>,
    #[account(mut)] pub reg_log: Account<'info, RegLog>,
    #[account(mut)] pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TournamentData { pub capacity: u16, pub registered: u16 }
#[account]
pub struct RegLog { pub players: Vec<Pubkey> }

#[error_code]
pub enum ErrorCode2 { #[msg("満員です。登録できません。" )] Full }

