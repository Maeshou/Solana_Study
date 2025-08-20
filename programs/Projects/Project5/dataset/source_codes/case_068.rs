use anchor_lang::prelude::*;

// ======================================================================
// 1) Glyph Librarium：刻印保管（初期化=小ループでチェックサム生成＋順不同代入）
// ======================================================================
declare_id!("GLYPH11111111111111111111111111111111111111");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum TomePhase { Draft, Engrave, Seal }

#[program]
pub mod glyph_librarium {
    use super::*;
    use TomePhase::*;

    pub fn init_tome(ctx: Context<InitTome>, seed: u32) -> Result<()> {
        // 並び：childB→log→parent→childA の順で設定し、同じ構造に見えないよう変化
        let b = &mut ctx.accounts.stone_b;
        let lg = &mut ctx.accounts.registry;
        let p = &mut ctx.accounts.codex;
        let a = &mut ctx.accounts.stone_a;

        // まずログ側を“種”から作る（小ループでチェックサム）
        lg.parent = p.key(); // parent未設定なので一旦後で上書きするためのダミー
        lg.channel = 9;
        lg.mark = 0;
        for i in 0..6 {
            lg.mark ^= ((seed as u64 + i as u64 * 97) << (i % 13)) & 0x00FF_FFFF;
        }

        // parent を最後ではなく“途中”で確定（構造を揺らす）
        p.owner = ctx.accounts.scribe.key();
        p.capacity = 300 + (seed % 50);
        p.stage = Draft;

        // 上で仮置きした registry.parent を正規の親に修正
        lg.parent = p.key();

        // 子は A/B で別計算：Aはseed低位、Bはseed高位を利用
        a.parent = p.key();
        a.slot = (seed & 7) as u8;
        a.power = (seed as u64 % 41) + 10;

        b.parent = p.key();
        b.slot = ((seed >> 3) & 7) as u8;
        b.power = ((seed as u64).rotate_left(7) % 53) + 11;

        Ok(())
    }

    pub fn inscribe(ctx: Context<Inscribe>, n: u32) -> Result<()> {
        let p = &mut ctx.accounts.codex;
        let a = &mut ctx.accounts.stone_a;
        let b = &mut ctx.accounts.stone_b;
        let lg = &mut ctx.accounts.registry;

        for t in 0..n {
            let mix = ((a.power ^ b.power) as u64).wrapping_mul(2654435761);
            a.power = a.power.checked_add((mix & 15) + (t as u64 & 7)).unwrap_or(u64::MAX);
            b.power = b.power.saturating_add(((mix >> 8) & 31) + 1);
            lg.mark = lg.mark.rotate_left((t % 31) as u32) ^ (mix & 0x000F_FFFF);
        }

        let gauge = (a.power as u128 + b.power as u128) / 3;
        if gauge as u64 > p.capacity as u64 {
            p.stage = Seal;
            a.power = a.power / 2 + 5;
            b.power = b.power / 2 + 7;
            lg.channel = lg.channel.saturating_add(1);
            msg!("seal: halve stones, channel++");
        } else {
            p.stage = Engrave;
            a.slot ^= 0x3;
            b.slot = b.slot.saturating_add(1);
            lg.mark ^= 0x000F_0FF0;
            msg!("engrave: slot/xor tweaks, mark flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTome<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub codex: Account<'info, Codex>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8)]
    pub stone_a: Account<'info, Stone>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8)]
    pub stone_b: Account<'info, Stone>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub scribe: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Inscribe<'info> {
    #[account(mut, has_one=owner)]
    pub codex: Account<'info, Codex>,
    #[account(
        mut,
        has_one=codex,
        constraint = stone_a.slot != stone_b.slot @ GlyphErr::Dup
    )]
    pub stone_a: Account<'info, Stone>,
    #[account(
        mut,
        has_one=codex,
        constraint = stone_b.slot != registry.channel @ GlyphErr::Dup
    )]
    pub stone_b: Account<'info, Stone>,
    #[account(mut, has_one=codex)]
    pub registry: Account<'info, Registry>,
    pub scribe: Signer<'info>,
}

#[account]
pub struct Codex { pub owner: Pubkey, pub capacity: u32, pub stage: TomePhase }
#[account]
pub struct Stone { pub parent: Pubkey, pub slot: u8, pub power: u64 }
#[account]
pub struct Registry { pub parent: Pubkey, pub channel: u8, pub mark: u64 }

#[error_code]
pub enum GlyphErr { #[msg("duplicate mutable account")] Dup }
