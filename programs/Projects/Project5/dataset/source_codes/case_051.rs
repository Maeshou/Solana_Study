
// 8) Battle Arena Dist — 正規近似（Weyl列の和）PDAあり
declare_id!("BAND888888888888888888888888888888888");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ArenaState { Queue, Fight, Pause }

#[program]
pub mod battle_arena_dist {
    use super::*;
    use ArenaState::*;

    pub fn init_arena(ctx: Context<InitArena>, cap: u32) -> Result<()> {
        let a = &mut ctx.accounts;
        a.board.judge = a.judge.key();
        a.board.cap = cap;
        a.board.state = Queue;
        Ok(())
    }

    pub fn duel(ctx: Context<Duel>, rounds: u32) -> Result<()> {
        let a = &mut ctx.accounts;

        for k in 0..rounds {
            // Weyl列 (k*phi) の分数部分（Q16）
            let phi_q16: u32 = 40503;
            let val = k.wrapping_mul(phi_q16);
            let frac_q16 = val & 0xFFFF;
            let contrib = (frac_q16 as u64) + 1;

            a.stats.sum = a.stats.sum.wrapping_add(contrib);
            a.stats.count = a.stats.count.wrapping_add(1);
            a.a.score = a.a.score.wrapping_add(((contrib >> 4) & 0x3FF) as u32);
            a.b.score = a.b.score.wrapping_add(((contrib >> 6) & 0x1FF) as u32);
        }

        if a.stats.sum > (a.board.cap as u64) * 100 {
            a.board.state = Pause;
            a.stats.flags = a.stats.flags.wrapping_add(2);
            a.b.score = a.b.score / 2 + 13;
            a.a.score = a.a.score + 7;
            msg!("pause: flags+2, adjust scores");
        } else {
            a.board.state = Fight;
            a.stats.sum = a.stats.sum.wrapping_mul(2);
            a.a.score = a.a.score.rotate_left(1);
            a.b.score = a.b.score.rotate_right(1);
            msg!("fight: sum*2, rotate scores");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitArena<'info> {
    #[account(init, payer=payer, space=8+32+4+1, seeds=[b"board", judge.key().as_ref()], bump)]
    pub board: Account<'info, BoardCfg>,
    #[account(init, payer=payer, space=8+32+4)]
    pub a: Account<'info, Fighter>,
    #[account(init, payer=payer, space=8+32+4)]
    pub b: Account<'info, Fighter>,
    #[account(init, payer=payer, space=8+8+4)]
    pub stats: Account<'info, ArenaStats>,
    #[account(mut)] pub payer: Signer<'info>,
    pub judge: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Duel<'info> {
    #[account(mut, seeds=[b"board", judge.key().as_ref()], bump, has_one=judge)]
    pub board: Account<'info, BoardCfg>,
    #[account(
        mut,
        constraint = a.key() != b.key() @ BandErr::Dup,
        constraint = a.key() != stats.key() @ BandErr::Dup
    )]
    pub a: Account<'info, Fighter>,
    #[account(
        mut,
        constraint = b.key() != stats.key() @ BandErr::Dup
    )]
    pub b: Account<'info, Fighter>,
    #[account(mut)]
    pub stats: Account<'info, ArenaStats>,
    pub judge: Signer<'info>,
}
#[account] pub struct BoardCfg { pub judge: Pubkey, pub cap: u32, pub state: ArenaState }
#[account] pub struct Fighter { pub user: Pubkey, pub score: u32 }
#[account] pub struct ArenaStats { pub sum: u64, pub count: u64, pub flags: u32 }
#[error_code] pub enum BandErr { #[msg("dup")] Dup }