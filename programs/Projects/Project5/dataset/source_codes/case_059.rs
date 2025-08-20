// ============================================================================
// 3) Sky Ladder — ラダー管理（PDAなし / has_one + seat不一致）
// ============================================================================
declare_id!("SKLD33333333333333333333333333333333333333333");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LadderState { Queue, Match, Freeze }

#[program]
pub mod sky_ladder {
    use super::*;
    use LadderState::*;

    pub fn init_ladder(ctx: Context<InitLadder>, limit: u32) -> Result<()> {
        let l = &mut ctx.accounts;
        l.board.judge = l.judge.key();
        l.board.limit = limit;
        l.board.state = Queue;

        l.a.board = l.board.key(); l.a.seat = 3;
        l.b.board = l.board.key(); l.b.seat = 7;
        l.tally.board = l.board.key(); l.tally.seat = 99;
        Ok(())
    }

    pub fn report(ctx: Context<Report>, hits_a: u32, hits_b: u32) -> Result<()> {
        let l = &mut ctx.accounts;

        for _ in 0..hits_a { l.a.score = l.a.score.saturating_add(4); l.tally.rounds = l.tally.rounds.saturating_add(1); }
        for _ in 0..hits_b { l.b.score = l.b.score.saturating_add(3); l.tally.rounds = l.tally.rounds.saturating_add(1); }

        let sum = l.a.score as u64 + l.b.score as u64;
        if sum > l.board.limit as u64 {
            l.board.state = Freeze;
            l.tally.matches = l.tally.matches.saturating_add(1);
            l.a.flags = l.a.flags.saturating_add(2);
            l.b.flags = l.b.flags.saturating_add(1);
            msg!("freeze: matches+1, flags on both");
        } else {
            l.board.state = Match;
            l.tally.matches = l.tally.matches.saturating_add(1);
            l.a.score = l.a.score.saturating_add(6);
            l.b.score = l.b.score.saturating_add(6);
            msg!("match: +scores, matches+1");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLadder<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub board: Account<'info, Board>,
    #[account(init, payer=payer, space=8+32+1+4+4)]
    pub a: Account<'info, Player>,
    #[account(init, payer=payer, space=8+32+1+4+4)]
    pub b: Account<'info, Player>,
    #[account(init, payer=payer, space=8+32+1+8+4)]
    pub tally: Account<'info, Tally>,
    #[account(mut)] pub payer: Signer<'info>,
    pub judge: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Report<'info> {
    #[account(mut, has_one=judge)]
    pub board: Account<'info, Board>,
    #[account(mut, has_one=board, constraint = a.seat != b.seat @ SkErr::Dup)]
    pub a: Account<'info, Player>,
    #[account(mut, has_one=board, constraint = b.seat != tally.seat @ SkErr::Dup)]
    pub b: Account<'info, Player>,
    #[account(mut, has_one=board)]
    pub tally: Account<'info, Tally>,
    pub judge: Signer<'info>,
}

#[account] pub struct Board { pub judge: Pubkey, pub limit: u32, pub state: LadderState }
#[account] pub struct Player { pub board: Pubkey, pub seat: u8, pub score: u32, pub flags: u32 }
#[account] pub struct Tally { pub board: Pubkey, pub seat: u8, pub rounds: u64, pub matches: u32 }
#[error_code] pub enum SkErr { #[msg("dup")] Dup }