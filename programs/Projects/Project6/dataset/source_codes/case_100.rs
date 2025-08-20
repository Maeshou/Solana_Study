// Rune Orchestra — 魔導オーケストラ：譜面ボードと演奏カード（席番号の不一致でType Cosplay遮断）
use anchor_lang::prelude::*;
declare_id!("RuNe0rChEsTrA99999000000000000000000000001");

#[program]
pub mod rune_orchestra {
    use super::*;
    use Part::*;

    /// オーケストラ初期化：
    /// - nameは64文字に制限し保存
    /// - seats_maskで使用可能な席集合をビットで指定
    /// - seedを混ぜて拍子テーブルをループ初期化
    pub fn init_orchestra(ctx: Context<InitOrchestra>, name: String, seed: u64, seats_mask: u32) -> Result<()> {
        let o = &mut ctx.accounts.orchestra;
        o.owner = ctx.accounts.maestro.key();

        let mut nm = name;
        if nm.len() > 64 { nm.truncate(64); }
        o.name = nm;

        o.seats_mask = seats_mask;
        o.revision = 1;
        o.mix = (seats_mask ^ (seed as u32)).rotate_left(7);

        let mut s = seed ^ 0xC3A5_C85C_97CB_3127u64;
        for i in 0..o.beat_table.len() {
            s = s.rotate_left(11) ^ (seats_mask as u64);
            o.beat_table[i] = ((s >> (i * 5)) as u16) & 0x3FFF;
        }
        Ok(())
    }

    /// 演奏カード初期化：
    /// - seat（席番号）の範囲／ビット許可を検証
    /// - partに応じて初期ポテンシャルを非線形生成
    pub fn init_score(ctx: Context<InitScore>, part: Part, seat: u8, tempo_bias: u16) -> Result<()> {
        let sc = &mut ctx.accounts.score;
        sc.orchestra = ctx.accounts.orchestra.key();

        require!(seat < 32, ErrCode::SeatOutOfRange);
        let allowed = (ctx.accounts.orchestra.seats_mask >> seat) & 1;
        require!(allowed == 1, ErrCode::SeatDisabled);

        sc.part = part;
        sc.seat = seat; // 一意フィールド
        sc.potential = 300 + ((tempo_bias as u32) % 500);

        for i in 0..sc.dynamics.len() {
            sc.dynamics[i] = ((sc.potential as u16) ^ ((i as u16) * 257)) & 0x7FFF;
        }
        Ok(())
    }

    /// 演奏処理：
    /// - actor と partner は同じ Part にはできても、同じ seat は不可（Cosplay遮断）
    /// - ループで譜面・ダイナミクスを更新、if/elseで分岐（各4行以上）
    pub fn perform(ctx: Context<Perform>, score_delta: i16, jitter: u16) -> Result<()> {
        let orch = &mut ctx.accounts.orchestra;
        let actor = &mut ctx.accounts.actor;
        let partner = &mut ctx.accounts.partner;
        let board = &mut ctx.accounts.board;

        // ループ：譜面の拍と外乱をブレンドして各演奏カードのダイナミクスを更新
        let mut acc: u32 = 0;
        for i in 0..orch.beat_table.len() {
            let local = ((orch.beat_table[i] as u32) ^ (jitter as u32).rotate_left((i as u32) & 15))
                .wrapping_add(((score_delta as i32).unsigned_abs()) & 0x3FFF);
            let idx = i % actor.dynamics.len();
            actor.dynamics[idx] =
                (actor.dynamics[idx] as u32 ^ (local & 0x7FFF)) as u16;
            acc = acc.wrapping_add(local & 0x1FFF);
        }

        // if/else：Partで分岐（各4行以上の副作用）
        if actor.part == Part::Conductor {
            actor.potential = actor.potential.saturating_add((acc & 0x0FFF) + (jitter as u32));
            orch.revision = orch.revision.saturating_add(1);
            board.lines = board.lines.saturating_add(1);
            board.hash = board.hash.wrapping_add((acc as u64).rotate_left(17));
            msg!("Conductor path: actor boosted, orchestra revised, board appended");
        } else {
            partner.potential = partner.potential.saturating_add(((acc >> 2) & 0x0FFF) + (score_delta.unsigned_abs() as u32));
            orch.mix = orch.mix.rotate_right(5) ^ ((partner.seat as u32) << 9);
            board.lines = board.lines.saturating_add(2);
            board.hash = board.hash ^ ((acc as u64).rotate_right(11));
            msg!("Non-Conductor path: partner boosted, mix updated, board branched");
        }

        // 追加ループ：譜面ボードのトレースをスパース更新
        for i in 0..board.trace.len() {
            board.trace[i] = board.trace[i].rotate_left((i as u32) & 7) ^ (orch.mix & 0xFFFF);
        }

        Ok(())
    }
}

// ───────── Accounts ─────────

#[derive(Accounts)]
pub struct InitOrchestra<'info> {
    #[account(init, payer = maestro, space = 8 + Orchestra::MAX)]
    pub orchestra: Account<'info, Orchestra>,
    #[account(mut)]
    pub maestro: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitScore<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub orchestra: Account<'info, Orchestra>,
    #[account(init, payer = player, space = 8 + ScoreCard::MAX)]
    pub score: Account<'info, ScoreCard>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Perform<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub orchestra: Account<'info, Orchestra>,

    #[account(mut, has_one = orchestra, owner = crate::ID)]
    pub board: Account<'info, ScoreBoard>,

    #[account(mut, has_one = orchestra, owner = crate::ID)]
    pub actor: Account<'info, ScoreCard>,

    #[account(
        mut,
        has_one = orchestra,
        owner = crate::ID,
        // 一意フィールド（seat）の不一致で同一口座の二重渡しを遮断
        constraint = actor.seat != partner.seat @ ErrCode::CosplayBlocked
    )]
    pub partner: Account<'info, ScoreCard>,

    pub owner: Signer<'info>,
}

// ───────── Data ─────────

#[account]
pub struct Orchestra {
    pub owner: Pubkey,
    pub name: String,
    pub seats_mask: u32,
    pub revision: u32,
    pub mix: u32,
    pub beat_table: [u16; 12],
}
impl Orchestra {
    pub const MAX: usize = 32 + 4 + 64 + 4 + 4 + 4 + 2 * 12;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Part { Conductor, Solo, Chorus }

#[account]
pub struct ScoreCard {
    pub orchestra: Pubkey,
    pub part: Part,
    pub seat: u8,          // 一意フィールド
    pub potential: u32,
    pub dynamics: [u16; 16],
}
impl ScoreCard {
    pub const MAX: usize = 32 + 1 + 1 + 4 + 2 * 16;
}

#[account]
pub struct ScoreBoard {
    pub orchestra: Pubkey,
    pub hash: u64,
    pub lines: u32,
    pub trace: [u32; 6],
}
impl ScoreBoard {
    pub const MAX: usize = 32 + 8 + 4 + 4 * 6;
}

#[error_code]
pub enum ErrCode {
    #[msg("Type Cosplay blocked by unique field (seat) mismatch")]
    CosplayBlocked,
    #[msg("Seat must be < 32")]
    SeatOutOfRange,
    #[msg("This seat is disabled by seats_mask")]
    SeatDisabled,
}
