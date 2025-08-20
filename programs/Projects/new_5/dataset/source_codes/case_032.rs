// ============================================================================
// 8) Scoring Arena (two mutable boards)
// ============================================================================
use anchor_lang::prelude::*;

declare_id!("ARENA888888888888888888888888888888888888");

#[program]
pub mod scoring_arena {
    use super::*;
    use BoardPhase::*;

    pub fn init_referee(ctx: Context<InitReferee>, seed: u32) -> Result<()> {
        let r = &mut ctx.accounts.referee;
        r.owner = ctx.accounts.owner.key();
        r.seed = seed;
        r.rng = seed ^ 0xA5A5_A5A5;
        r.rounds = 0;
        Ok(())
    }

    pub fn init_board(ctx: Context<InitBoard>, slot: u16) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.parent = ctx.accounts.referee.key();
        b.slot = slot;
        b.phase = Warmup;
        b.score = 0;
        b.combos = 0;
        Ok(())
    }

    pub fn tally_pair(ctx: Context<TallyPair>, boost: u32) -> Result<()> {
        let r = &mut ctx.accounts.referee;
        let b1 = &mut ctx.accounts.a_board;
        let b2 = &mut ctx.accounts.b_board;

        // RNG-ish loop
        for _ in 0..5 {
            r.rng = r.rng.rotate_left(5) ^ (r.seed.rotate_right(3));
            r.rounds = r.rounds.saturating_add(1);
        }

        if (r.rng & 1) == 1 {
            b1.phase = Active;
            b1.score = b1.score.saturating_add(boost + (r.rounds as u32 & 31));
            b1.combos = b1.combos.saturating_add(2);
            r.seed = r.seed ^ 0x5F5F_5F5F;
            msg!("B1 active; score={}, combos={}", b1.score, b1.combos);
        } else {
            b1.phase = Cooldown;
            b1.score = b1.score / 2 + (b1.combos & 7);
            b1.combos = b1.combos.saturating_sub(1);
            r.seed = r.seed / 3 + 17;
            msg!("B1 cool; score={}, combos={}", b1.score, b1.combos);
        }

        for _ in 0..4 {
            if (b2.slot as u32 + r.rounds as u32) & 3 == 0 {
                b2.phase = Active;
                b2.combos = b2.combos.saturating_add(1);
                b2.score = b2.score.saturating_add(5 + (r.seed & 15));
                msg!("B2 chain+; score={}, combos={}", b2.score, b2.combos);
            } else {
                b2.phase = Warmup;
                b2.score = b2.score.saturating_sub(b2.score.min(3));
                r.rng = r.rng ^ (b2.score as u32);
                msg!("B2 warm; score={}, rng={}", b2.score, r.rng);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitReferee<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 4 + 8)]
    pub referee: Account<'info, Referee>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(mut)]
    pub referee: Account<'info, Referee>,
    #[account(init, payer = maker, space = 8 + 32 + 2 + 1 + 4 + 4)]
    pub board: Account<'info, Board>,
    #[account(mut)]
    pub maker: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TallyPair<'info> {
    #[account(mut)]
    pub referee: Account<'info, Referee>,
    #[account(mut, has_one = parent)]
    pub a_board: Account<'info, Board>,
    #[account(mut, has_one = parent)]
    pub b_board: Account<'info, Board>, // can alias
}

#[account]
pub struct Referee {
    pub owner: Pubkey,
    pub seed: u32,
    pub rng: u32,
    pub rounds: u64,
}

#[account]
pub struct Board {
    pub parent: Pubkey,
    pub slot: u16,
    pub phase: BoardPhase,
    pub score: u32,
    pub combos: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum BoardPhase {
    Warmup,
    Active,
    Cooldown,
}
use BoardPhase::*;

#[error_code]
pub enum ArenaError {
    #[msg("score error")]
    ScoreError,
}
