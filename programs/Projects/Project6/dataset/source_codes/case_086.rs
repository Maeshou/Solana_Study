// =======================================
// (1) Guild Chronicle — ギルド掲示板と役割カード
// 役割の異なる MemberCard 同士を同一Pubkeyで差し替えできないよう、has_one/owner と
// 役割フィールド不一致制約で多層防御。
// =======================================

use anchor_lang::prelude::*;
declare_id!("GUiLdChr0n1cLe11111111111111111111111111111");

#[program]
pub mod guild_chronicle {
    use super::*;
    use MemberRole::*;

    pub fn init_guild(ctx: Context<InitGuild>, name: String) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        guild.owner = ctx.accounts.admin.key();
        guild.name = name;
        guild.total_posts = 0;
        guild.flags = 0;
        Ok(())
    }

    pub fn init_member_card(ctx: Context<InitMemberCard>, role: MemberRole) -> Result<()> {
        let card = &mut ctx.accounts.card;
        card.guild = ctx.accounts.guild.key();
        card.role = role;
        card.reputation = 0;
        card.active = true;
        card.counters = [0; 8];
        Ok(())
    }

    pub fn update_post_flow(
        ctx: Context<UpdatePostFlow>,
        weight: u16,
        boost: u16,
    ) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        let actor = &mut ctx.accounts.actor;
        let reviewer = &mut ctx.accounts.reviewer;
        let board = &mut ctx.accounts.board;

        // ── ループ（移動平均っぽい重み付けカウント） ──
        let mut acc: u64 = 0;
        for i in 0..actor.counters.len() {
            // rotate + xor + クリップ
            let rotated = actor.counters[i].rotate_left((i as u32) & 7);
            let mixed = rotated ^ ((boost as u64) << (i as u32 & 15));
            let next = mixed.checked_add(weight as u64).unwrap_or(u64::MAX);
            actor.counters[i] = if next > 1_000_000 { 1_000_000 } else { next };
            acc = acc.saturating_add(actor.counters[i] & 0xFFFF);
        }

        // ── 分岐（if/else：各ブロック4行以上） ──
        if actor.role == Leader {
            actor.reputation = actor.reputation.saturating_add((acc % 97) as u32);
            guild.total_posts = guild.total_posts.saturating_add(1);
            board.last_hash = board.last_hash.wrapping_mul(1_146_295).wrapping_add(acc as u64);
            board.queue_len = board.queue_len.saturating_add(1);
            msg!("Leader actor updated reputation and queued a post");
        } else {
            reviewer.reputation = reviewer.reputation.saturating_add(((acc >> 2) % 53) as u32);
            guild.flags = guild.flags | 0b10;
            board.queue_len = board.queue_len.saturating_add(2);
            board.last_hash = board.last_hash ^ (acc as u64).rotate_right(7);
            msg!("Non-leader path: reviewer reputation adjusted and board queued");
        }

        Ok(())
    }
}

// ───────────────── Accounts ─────────────────

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = admin, space = 8 + Guild::MAX_SIZE)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitMemberCard<'info> {
    #[account(mut, has_one = owner)]
    pub guild: Account<'info, Guild>,
    #[account(
        init,
        payer = user,
        space = 8 + MemberCard::MAX_SIZE
    )]
    pub card: Account<'info, MemberCard>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// ギルドは本プログラムが所有（型＋owner二重フェンス）
    #[account(address = crate::ID)]
    pub program_id_alias: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePostFlow<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub guild: Account<'info, Guild>,

    #[account(
        mut,
        has_one = guild,
        owner = crate::ID
    )]
    pub board: Account<'info, Board>,

    #[account(
        mut,
        has_one = guild,
        owner = crate::ID
    )]
    pub actor: Account<'info, MemberCard>,

    #[account(
        mut,
        has_one = guild,
        owner = crate::ID,
        // 一意フィールド（role）の不一致で同一口座の二重渡しをブロック
        constraint = actor.role != reviewer.role @ ErrCode::CosplayBlocked
    )]
    pub reviewer: Account<'info, MemberCard>,

    pub owner: Signer<'info>,
}

// ───────────────── Data ─────────────────

#[account]
pub struct Guild {
    pub owner: Pubkey,
    pub name: String,
    pub total_posts: u64,
    pub flags: u32,
}
impl Guild {
    pub const MAX_SIZE: usize = 32 + 4 + 64 /*name*/ + 8 + 4;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum MemberRole {
    Leader,
    Officer,
    Member,
}
use MemberRole::*;

#[account]
pub struct MemberCard {
    pub guild: Pubkey,
    pub role: MemberRole,
    pub reputation: u32,
    pub active: bool,
    pub counters: [u64; 8],
}
impl MemberCard {
    pub const MAX_SIZE: usize = 32 + 1 + 4 + 1 + (8 * 8);
}

#[account]
pub struct Board {
    pub guild: Pubkey,
    pub last_hash: u64,
    pub queue_len: u32,
}
impl Board {
    pub const MAX_SIZE: usize = 32 + 8 + 4;
}

#[error_code]
pub enum ErrCode {
    #[msg("Type Cosplay blocked by role mismatch constraint")]
    CosplayBlocked,
}