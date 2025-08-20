// (1) Guild Ledger — ギルド管理（メンバー登録と活動ポイント集計）
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");

#[program]
pub mod guild_ledger {
    use super::*;
    use Rank::*;

    pub fn init_guild(ctx: Context<InitGuild>, name_hash: u64) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        g.leader = ctx.accounts.leader.key();
        g.name_hash = name_hash;
        g.member_count = 0;
        g.activity_sum = 0;
        Ok(())
    }

    pub fn register_member(ctx: Context<RegisterMember>, nickname_hash: u32, rank: Rank) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        let m = &mut ctx.accounts.member;
        m.guild = g.key();
        m.nickname_hash = nickname_hash;
        m.rank = rank;
        m.points = 0;
        g.member_count = g.member_count.saturating_add(1);
        Ok(())
    }

    pub fn process_week(ctx: Context<ProcessWeek>, contributions: Vec<u64>) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        let actor = &mut ctx.accounts.actor;
        let target = &mut ctx.accounts.target;
        let board = &mut ctx.accounts.activity_board;

        // ループ：移動平均・XOR・ビット回転で簡易特徴量
        let mut sum: u128 = 0;
        let mut mix: u64 = 0;
        for c in contributions {
            let clipped = c.min(100_000);
            sum = sum.saturating_add(clipped as u128);
            mix = (mix ^ clipped).rotate_left(3);
        }
        let avg = if contributions.is_empty() { 0 } else { (sum / contributions.len() as u128) as u64 };

        // 分岐（4行以上/枝）
        if actor.rank == Officer {
            actor.points = actor.points.saturating_add(avg + (mix & 0x3FF));
            target.points = target.points.saturating_add(avg / 2);
            g.activity_sum = g.activity_sum.saturating_add(actor.points).saturating_add(target.points);
            msg!("Officer path: avg={}, mix={}, actor={}, target={}", avg, mix, actor.points, target.points);
        } else {
            actor.points = actor.points.saturating_add(avg / 2);
            target.points = target.points.saturating_add(avg + ((mix >> 2) & 0x1FF));
            g.activity_sum = g.activity_sum.saturating_add(actor.points).saturating_add(target.points);
            msg!("Non-officer path: avg={}, mix={}, actor={}, target={}", avg, mix, actor.points, target.points);
        }

        // ニュートン法で平方根近似→メーター
        let mut x = (g.activity_sum as u128).max(1);
        let mut i = 0;
        while i < 3 {
            x = (x + (g.activity_sum as u128 / x)).max(1) / 2;
            i += 1;
        }
        board.guild = g.key();
        board.meter = (x as u64).min(1_000_000);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = leader, space = 8 + 32 + 8 + 4 + 8)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterMember<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1 + 8)]
    pub member: Account<'info, Member>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Type Cosplay対策：同一ギルド配下 + 役割不一致で二重渡しを遮断
#[derive(Accounts)]
pub struct ProcessWeek<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    #[account(mut, has_one = guild)]
    pub activity_board: Account<'info, ActivityBoard>,
    #[account(
        mut,
        has_one = guild,
        constraint = actor.rank != target.rank @ ErrCode::CosplayBlocked
    )]
    pub actor: Account<'info, Member>,
    #[account(mut, has_one = guild)]
    pub target: Account<'info, Member>,
}

#[account]
pub struct Guild {
    pub leader: Pubkey,
    pub name_hash: u64,
    pub member_count: u32,
    pub activity_sum: u64,
}

#[account]
pub struct Member {
    pub guild: Pubkey,
    pub nickname_hash: u32,
    pub rank: Rank,
    pub points: u64,
}

#[account]
pub struct ActivityBoard {
    pub guild: Pubkey,
    pub meter: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Rank {
    Leader,
    Officer,
    Member,
}

#[error_code]
pub enum ErrCode {
    #[msg("Type cosplay prevented: role mismatch required.")]
    CosplayBlocked,
}
