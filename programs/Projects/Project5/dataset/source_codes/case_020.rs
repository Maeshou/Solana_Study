
// ============================================================================
// 5) Coliseum Duel（コロシアム対戦）— PDA使用 + seeds二系統 + constraint（同型二重禁止）
//    防止法: fighter_a≠fighter_b を属性で強制、boardをPDAで固定
// ============================================================================
declare_id!("COLI55555555555555555555555555555555");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ArenaState { Halt, Running }

#[program]
pub mod coliseum_duel {
    use super::*;

    pub fn init_coliseum(ctx: Context<InitColiseum>, limit: u32) -> Result<()> {
        ctx.accounts.arena.owner = ctx.accounts.host.key();
        ctx.accounts.arena.limit = limit;
        ctx.accounts.arena.state = ArenaState::Running;

        ctx.accounts.board.total = 0;
        ctx.accounts.board.matches = 0;

        ctx.accounts.config.rounds = 3;
        ctx.accounts.config.strict = 1;
        Ok(())
    }

    pub fn duel(ctx: Context<Duel>, a_hits: u32, b_hits: u32) -> Result<()> {
        // ループ（二人分）
        for _ in 0..a_hits {
            ctx.accounts.fighter_a.score = ctx.accounts.fighter_a.score.saturating_add(1);
            ctx.accounts.board.total = ctx.accounts.board.total.saturating_add(1);
        }
        for _ in 0..b_hits {
            ctx.accounts.fighter_b.score = ctx.accounts.fighter_b.score.saturating_add(1);
            ctx.accounts.board.total = ctx.accounts.board.total.saturating_add(1);
        }

        // 分岐
        if ctx.accounts.board.total > ctx.accounts.arena.limit as u64 {
            ctx.accounts.arena.state = ArenaState::Halt;
            ctx.accounts.board.matches = ctx.accounts.board.matches.saturating_add(1);
            ctx.accounts.config.strict = 1;
            msg!("limit exceeded; halt");
        } else {
            ctx.accounts.arena.state = ArenaState::Running;
            ctx.accounts.board.matches = ctx.accounts.board.matches.saturating_add(1);
            ctx.accounts.config.strict = 0;
            msg!("ok; continue");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitColiseum<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub arena: Account<'info, Arena>,
    #[account(init, payer=payer, space=8+8+4, seeds=[b"board", payer.key().as_ref()], bump)]
    pub board: Account<'info, ScoreBoard>,
    #[account(init, payer=payer, space=8+4+1, seeds=[b"cfg", payer.key().as_ref()], bump)]
    pub config: Account<'info, MatchCfg>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Duel<'info> {
    #[account(mut)]
    pub arena: Account<'info, Arena>,
    #[account(mut, constraint = fighter_a.key() != fighter_b.key(), error = DuelErr::Same)]
    pub fighter_a: Account<'info, Fighter>,
    #[account(mut)]
    pub fighter_b: Account<'info, Fighter>,
    #[account(mut, seeds=[b"board", payer.key().as_ref()], bump)]
    pub board: Account<'info, ScoreBoard>,
    /// CHECK: seeds固定
    pub payer: UncheckedAccount<'info>,
    #[account(mut, seeds=[b"cfg", payer.key().as_ref()], bump)]
    pub config: Account<'info, MatchCfg>,
}

#[account] pub struct Arena { pub owner: Pubkey, pub limit: u32, pub state: ArenaState }
#[account] pub struct Fighter { pub owner: Pubkey, pub score: u32, pub alive: u8 }
#[account] pub struct ScoreBoard { pub total: u64, pub matches: u32 }
#[account] pub struct MatchCfg { pub rounds: u32, pub strict: u8 }
#[error_code] pub enum DuelErr { #[msg("dup")] Same }
