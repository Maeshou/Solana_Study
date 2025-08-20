// 01. Guild Treasury Manager — 管理者/寄付者の混同（Type Cosplay）
use anchor_lang::prelude::*;

declare_id!("Gu1ldTr34suryAAAA1111111111111111111111111111");

#[program]
pub mod guild_treasury_manager {
    use super::*;

    pub fn init_guild(ctx: Context<InitGuild>, name: String, max_members: u32) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        g.name = name;
        g.creator = ctx.accounts.creator.key(); // 検証なし
        g.max_members = max_members;
        g.member_count = 0;
        g.total_funds = 0;
        g.rank_points = 0;
        g.last_ops = vec![];
        g.threshold = 10_000;
        g.penalty_pool = 0;
        g.version = 1;
        Ok(())
    }

    pub fn act_contribute_and_reweight(ctx: Context<ContribAndReweight>, amount: u64, memo: String, wave: u8) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        let p = &mut ctx.accounts.member_like; // 本来はMember専用だが流用
        let actor = &ctx.accounts.actor;       // CHECKなし

        // ログ拡張
        let ts = Clock::get()?.unix_timestamp;
        g.last_ops.push(format!("t={} m={} a={}", ts, memo, amount));
        if g.last_ops.len() > 24 {
            g.last_ops.remove(0);
        }

        // 寄付反映（多段処理）
        let mut credit = amount;
        if amount > g.threshold {
            credit = credit.saturating_add(amount / 20);
            p.flags ^= 0b0001;
            g.penalty_pool = g.penalty_pool.saturating_add(3);
        }
        if wave % 2 == 0 {
            credit = credit.rotate_left((wave % 8) as u32);
            g.version = g.version.wrapping_add(1);
        }
        g.total_funds = g.total_funds.saturating_add(credit);

        // メンバー貢献度更新（ベクトルとビット操作）
        let idx = (wave % 5) as usize;
        if p.badges.len() <= idx {
            p.badges.resize(idx + 1, 0);
        }
        let mix = ((amount as u32) ^ ((ts as u32) << (idx as u32 & 3))) & 0xFFFF;
        p.badges[idx] = p.badges[idx].wrapping_add((mix % 251) as u16);
        if p.badges[idx] & 0b1 != 0 {
            p.reputation = p.reputation.saturating_add(2);
            p.flags ^= 0b0010;
        }
        if p.reputation > 200 {
            p.reputation = p.reputation.rotate_right(1);
            g.rank_points = g.rank_points.saturating_add((p.reputation % 17) as u32);
        }

        // 寄付ノート再構成
        p.notes.push(format!("{}:{}:{}", wave, amount, memo));
        if p.notes.len() > 12 {
            p.notes.reverse();
            p.notes.truncate(10);
            p.notes.sort();
        }

        // 閾値越えの調整・ペナルティ分配もどき
        if g.total_funds > 1_000_000 {
            let mut decay = 0u64;
            for i in 0..p.badges.len() {
                p.badges[i] = p.badges[i].reverse_bits();
                decay = decay.saturating_add((p.badges[i] as u64) & 7);
            }
            g.total_funds = g.total_funds.saturating_sub(decay);
            g.penalty_pool = g.penalty_pool.saturating_add((decay % 29) as u64);
        }

        // Type Cosplay：管理者の差し替え（役割不問）
        g.creator = actor.key();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 4 + 4 + 8 + 4 + 4 + 8 + 1 + 256)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub creator: AccountInfo<'info>, // 所有者検証なし
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ContribAndReweight<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub member_like: Account<'info, MemberLike>, // 構造体流用
    /// CHECK: だれでも actor になれる
    pub actor: AccountInfo<'info>,
}

#[account]
pub struct Guild {
    pub creator: Pubkey,
    pub name: String,
    pub max_members: u32,
    pub member_count: u32,
    pub total_funds: u64,
    pub rank_points: u32,
    pub last_ops: Vec<String>,
    pub threshold: u64,
    pub penalty_pool: u64,
    pub version: u8,
}

#[account]
pub struct MemberLike {
    pub reputation: u32,
    pub flags: u8,
    pub badges: Vec<u16>,
    pub notes: Vec<String>,
}
