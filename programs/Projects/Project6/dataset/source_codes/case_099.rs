// (2) Sky Circuit — 空路サーキットと機体カード（レーン不一致で遮断）
use anchor_lang::prelude::*;
declare_id!("SkYc1rcu1tB888880000000000000000000000002");

#[program]
pub mod sky_circuit {
    use super::*;
    use Craft::*;

    pub fn init_track(ctx: Context<InitTrack>, title: String, lanes_mask: u32, seed: u64) -> Result<()> {
        let t = &mut ctx.accounts.track;
        t.owner = ctx.accounts.marshal.key();

        // タイトルは64文字に制限して保存
        let mut tt = title;
        if tt.len() > 64 { tt.truncate(64); }
        t.title = tt;

        // 使用可能レーン集合
        t.lanes_mask = lanes_mask;

        // 乱数風初期化で風向きテーブルを作る
        let mut s = seed ^ 0xD6E8FEB86659FD93u64;
        for i in 0..t.wind.len() {
            s = s.rotate_left(13) ^ (lanes_mask as u64);
            t.wind[i] = ((s >> (i * 5)) as u16) & 0x3FFF;
        }

        t.epoch = ((seed as u32) % 997 + 1) as u16;
        t.mix = (lanes_mask ^ (seed as u32)) & 0xFFFF_FFFF;
        Ok(())
    }

    pub fn init_craft(ctx: Context<InitCraft>, craft: Craft, lane: u8, bias: u16) -> Result<()> {
        let c = &mut ctx.accounts.craft_card;
        c.track = ctx.accounts.track.key();

        // lane検証：0..31 かつ lanes_mask で許可されていること
        require!(lane < 32, ErrCode::LaneOutOfRange);
        let allowed = (ctx.accounts.track.lanes_mask >> lane) & 1;
        require!(allowed == 1, ErrCode::LaneDisabled);

        c.kind = craft;
        c.lane = lane; // 一意フィールド
        c.energy = 500 + (bias as u32 % 300);
        for i in 0..c.telemetry.len() {
            c.telemetry[i] = ((c.energy as u16) ^ ((i as u16) * 123)) & 0x7FFF;
        }
        Ok(())
    }

    pub fn fly_epoch(ctx: Context<FlyEpoch>, drift: u16, factor: u16) -> Result<()> {
        let tr = &mut ctx.accounts.track;
        let a = &mut ctx.accounts.actor;
        let r = &mut ctx.accounts.rival;
        let l = &mut ctx.accounts.log;

        // ループ：風向きとドリフトをブレンドして各機体のテレメトリ更新
        let mut acc: u32 = 0;
        for i in 0..tr.wind.len() {
            let local = ((tr.wind[i] as u32) ^ (drift as u32).rotate_left((i as u32) & 15))
                .wrapping_add((factor as u32) * ((i as u32) + 1));
            a.telemetry[i % a.telemetry.len()] =
                (a.telemetry[i % a.telemetry.len()] as u32 ^ (local & 0x7FFF)) as u16;
            acc = acc.wrapping_add(local & 0x3FFF);
        }

        // 分岐：機体の種類でパス分け（各4行以上）
        if a.kind == Falcon {
            a.energy = a.energy.saturating_add((acc & 0x1FFF) + (factor as u32));
            tr.mix = tr.mix.rotate_left(5) ^ ((a.lane as u32) << 7);
            l.rounds = l.rounds.saturating_add(1);
            l.hash = l.hash.wrapping_add((acc as u64).rotate_left(17));
            msg!("Falcon path: energy boosted and track mixed");
        } else {
            r.energy = r.energy.saturating_add(((acc >> 2) & 0x0FFF) + (drift as u32));
            tr.mix = tr.mix.rotate_right(3) ^ ((r.lane as u32) << 9);
            l.rounds = l.rounds.saturating_add(2);
            l.hash = l.hash ^ ((acc as u64).rotate_right(11));
            msg!("Non-Falcon path: rival boosted and track mixed differently");
        }

        // 追加ループ：ログのスパース更新
        for i in 0..l.trace.len() {
            l.trace[i] = l.trace[i].rotate_left((i as u32) & 7) ^ (tr.mix & 0xFFFF);
        }
        Ok(())
    }
}

// ───────── Accounts ─────────

#[derive(Accounts)]
pub struct InitTrack<'info> {
    #[account(init, payer = marshal, space = 8 + Track::MAX)]
    pub track: Account<'info, Track>,
    #[account(mut)]
    pub marshal: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitCraft<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub track: Account<'info, Track>,
    #[account(init, payer = pilot, space = 8 + CraftCard::MAX)]
    pub craft_card: Account<'info, CraftCard>,
    #[account(mut)]
    pub pilot: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FlyEpoch<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub track: Account<'info, Track>,
    #[account(mut, has_one = track, owner = crate::ID)]
    pub log: Account<'info, FlightLog>,
    #[account(mut, has_one = track, owner = crate::ID)]
    pub actor: Account<'info, CraftCard>,
    #[account(
        mut,
        has_one = track,
        owner = crate::ID,
        // 一意フィールド（lane）不一致で同一口座の二重渡しを遮断
        constraint = actor.lane != rival.lane @ ErrCode::CosplayBlocked
    )]
    pub rival: Account<'info, CraftCard>,
    pub owner: Signer<'info>,
}

// ───────── Data ─────────

#[account]
pub struct Track {
    pub owner: Pubkey,
    pub title: String,
    pub lanes_mask: u32,
    pub epoch: u16,
    pub mix: u32,
    pub wind: [u16; 12],
}
impl Track {
    pub const MAX: usize = 32 + 4 + 64 + 4 + 2 + 4 + 2 * 12;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Craft { Falcon, Wasp, Rhino }
use Craft::*;

#[account]
pub struct CraftCard {
    pub track: Pubkey,
    pub kind: Craft,
    pub lane: u8,        // 一意フィールド
    pub energy: u32,
    pub telemetry: [u16; 16],
}
impl CraftCard {
    pub const MAX: usize = 32 + 1 + 1 + 4 + 2 * 16;
}

#[account]
pub struct FlightLog {
    pub track: Pubkey,
    pub hash: u64,
    pub rounds: u32,
    pub trace: [u32; 6],
}
impl FlightLog {
    pub const MAX: usize = 32 + 8 + 4 + 4 * 6;
}

#[error_code]
pub enum ErrCode {
    #[msg("Type Cosplay blocked by unique field mismatch")]
    CosplayBlocked,
    #[msg("Lane index must be < 32")]
    LaneOutOfRange,
    #[msg("This lane is disabled by lanes_mask")]
    LaneDisabled,
}
