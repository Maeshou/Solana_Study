// ======================================================================
// 5) Chess Arena：チェスクラブ（初期化＝Zobrist風混合で初期スコア）
// ======================================================================
declare_id!("CHES55555555555555555555555555555555555555");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum TourPhase { Signup, Round, Closed }

#[program]
pub mod chess_arena {
    use super::*;
    use TourPhase::*;

    pub fn init_club(ctx: Context<InitClub>, seed: u32) -> Result<()> {
        let cl = &mut ctx.accounts.club;
        cl.owner = ctx.accounts.director.key();
        cl.rating_cap = seed * 5 + 800;
        cl.phase = Signup;

        let a = &mut ctx.accounts.board_a;
        let b = &mut ctx.accounts.board_b;
        let ar = &mut ctx.accounts.arbiter;

        let h1 = seed ^ 0x9E37_79B9;
        let h2 = seed.rotate_left(7) ^ 0x1234_5678;

        a.club = cl.key(); a.table = (seed & 7) as u8; a.score = (h1 as u32 & 1023) + 30;
        b.club = cl.key(); b.table = ((seed >> 2) & 7) as u8; b.score = ((h2 as u32 >> 5) & 1023) + 27;

        ar.club = cl.key(); ar.table = 9; ar.rounds = 0; ar.seed = seed ^ 0xBEEF_CAFE;
        Ok(())
    }

    pub fn pair(ctx: Context<PairRound>, rounds: u32) -> Result<()> {
        let cl = &mut ctx.accounts.club;
        let a = &mut ctx.accounts.board_a;
        let b = &mut ctx.accounts.board_b;
        let ar = &mut ctx.accounts.arbiter;

        for i in 0..rounds {
            let z = ((a.score ^ b.score) as u64).wrapping_mul(1099511628211);
            a.score = a.score.checked_add(((z & 63) as u32) + 3).unwrap_or(u32::MAX);
            b.score = b.score.saturating_add((((z >> 6) & 63) as u32) + 5);
            ar.rounds = ar.rounds.saturating_add(1);
            ar.seed ^= (z as u32).rotate_left((i % 17) as u32);
        }

        let mean = if ar.rounds == 0 { 0 } else { ((a.score + b.score) as u64 / ar.rounds) as u32 };
        if mean > cl.rating_cap {
            cl.phase = Closed;
            a.table ^= 1; b.table = b.table.saturating_add(1);
            ar.table = ar.table.saturating_add(1);
            msg!("closed: table tweaks & arbiter move");
        } else {
            cl.phase = Round;
            a.score = a.score.saturating_add(9);
            b.score = b.score / 2 + 11;
            ar.seed ^= 0x0F0F_F0F0;
            msg!("round: score adjust & seed flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitClub<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub club: Account<'info, Club>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub board_a: Account<'info, Board>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub board_b: Account<'info, Board>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 4)]
    pub arbiter: Account<'info, ArbiterTape>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub director: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PairRound<'info> {
    #[account(mut, has_one=owner)]
    pub club: Account<'info, Club>,
    #[account(
        mut,
        has_one=club,
        constraint = board_a.table != board_b.table @ ChesErr::Dup
    )]
    pub board_a: Account<'info, Board>,
    #[account(
        mut,
        has_one=club,
        constraint = board_b.table != arbiter.table @ ChesErr::Dup
    )]
    pub board_b: Account<'info, Board>,
    #[account(mut, has_one=club)]
    pub arbiter: Account<'info, ArbiterTape>,
    pub director: Signer<'info>,
}

#[account] pub struct Club { pub owner: Pubkey, pub rating_cap: u32, pub phase: TourPhase }
#[account] pub struct Board { pub club: Pubkey, pub table: u8, pub score: u32 }
#[account] pub struct ArbiterTape { pub club: Pubkey, pub table: u8, pub rounds: u64, pub seed: u32 }

#[error_code] pub enum ChesErr { #[msg("duplicate mutable account")] Dup }
