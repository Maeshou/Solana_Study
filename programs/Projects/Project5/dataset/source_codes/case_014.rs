// ============================================================================
// 5) ArenaClash（闘技場）— PDA使用 + seeds二系統で一意化 + constraint
// ============================================================================
declare_id!("ARNA55555555555555555555555555555555");

#[program]
pub mod arena_clash {
    use super::*;

    pub fn init_arena(ctx: Context<InitArena>, limit: u32) -> Result<()> {
        ctx.accounts.arena.host = ctx.accounts.organizer.key();
        ctx.accounts.arena.limit = limit;
        ctx.accounts.arena.open = true;

        ctx.accounts.board.total = 0;
        ctx.accounts.board.fights = 0;

        ctx.accounts.match_cfg.rounds = 3;
        ctx.accounts.match_cfg.strict = true;
        Ok(())
    }

    pub fn duel(ctx: Context<Duel>, hits_a: u32, hits_b: u32) -> Result<()> {
        require!(ctx.accounts.fighter_a.key() != ctx.accounts.fighter_b.key(), ArenaErr::Dup);
        require!(ctx.accounts.arena.key() != ctx.accounts.board.key(), ArenaErr::Dup);

        for _ in 0..hits_a {
            ctx.accounts.fighter_a.score = ctx.accounts.fighter_a.score.saturating_add(1);
            ctx.accounts.board.total = ctx.accounts.board.total.saturating_add(1);
        }
        for _ in 0..hits_b {
            ctx.accounts.fighter_b.score = ctx.accounts.fighter_b.score.saturating_add(1);
            ctx.accounts.board.total = ctx.accounts.board.total.saturating_add(1);
        }

        if ctx.accounts.board.total > ctx.accounts.arena.limit as u64 {
            ctx.accounts.arena.open = false;
            ctx.accounts.board.fights = ctx.accounts.board.fights.saturating_add(1);
            ctx.accounts.match_cfg.strict = true;
            msg!("limit exceeded");
        } else {
            ctx.accounts.arena.open = true;
            ctx.accounts.board.fights = ctx.accounts.board.fights.saturating_add(1);
            ctx.accounts.match_cfg.strict = false;
            msg!("ok");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitArena<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub arena: Account<'info, Arena>,
    #[account(init, payer=payer, space=8+8+4, seeds=[b"board", payer.key().as_ref()], bump)]
    pub board: Account<'info, Board>,
    #[account(init, payer=payer, space=8+4+1, seeds=[b"cfg", payer.key().as_ref()], bump)]
    pub match_cfg: Account<'info, MatchCfg>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Duel<'info> {
    #[account(mut)]
    pub arena: Account<'info, Arena>,
    #[account(mut)]
    pub fighter_a: Account<'info, Fighter>,
    #[account(mut, constraint = fighter_a.key() != fighter_b.key(), error = ArenaErr::Dup)]
    pub fighter_b: Account<'info, Fighter>,
    #[account(mut, seeds=[b"board", payer.key().as_ref()], bump)]
    pub board: Account<'info, Board>,
    /// CHECK: seeds固定用
    pub payer: UncheckedAccount<'info>,
    #[account(mut, seeds=[b"cfg", payer.key().as_ref()], bump)]
    pub match_cfg: Account<'info, MatchCfg>,
}

#[account] pub struct Arena { pub host: Pubkey, pub limit: u32, pub open: bool }
#[account] pub struct Fighter { pub owner: Pubkey, pub score: u32, pub alive: bool }
#[account] pub struct Board { pub total: u64, pub fights: u32 }
#[account] pub struct MatchCfg { pub rounds: u32, pub strict: bool }

#[error_code] pub enum ArenaErr { #[msg("dup")] Dup }
