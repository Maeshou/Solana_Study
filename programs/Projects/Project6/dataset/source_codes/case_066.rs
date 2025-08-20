// (1) BadgeCraft — バッジ工房（クラフター同士の役割不一致でなりすまし阻止）
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");

#[program]
pub mod badge_craft {
    use super::*;
    use CraftRole::*;

    pub fn init_board(ctx: Context<InitBoard>, series: u32) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.admin = ctx.accounts.admin.key();
        b.series = series;
        b.badges = 0;
        Ok(())
    }

    pub fn enroll_crafter(ctx: Context<EnrollCrafter>, role: CraftRole, tier: u8) -> Result<()> {
        let b = &mut ctx.accounts.board;
        let c = &mut ctx.accounts.crafter;
        c.board = b.key();
        c.role = role;
        c.tier = tier;
        c.score = 0;
        Ok(())
    }

    pub fn mint_badge(ctx: Context<MintBadge>, materials: Vec<u16>) -> Result<()> {
        let b = &mut ctx.accounts.board;
        let a = &mut ctx.accounts.actor;
        let p = &mut ctx.accounts.partner;
        let l = &mut ctx.accounts.log;

        // ループ：素材を集計＆ビットミックス
        let mut sum: u32 = 0;
        let mut mix: u16 = 0;
        for m in materials {
            let v = (m & 0x3FF) as u32;
            sum = sum.saturating_add(v);
            mix ^= ((m << 1) | (m >> 1)) & 0x3FF;
        }
        let base = sum + (mix as u32 & 0xFF);

        // if/else（各枝4行以上）
        if a.role == Master {
            a.score = a.score.saturating_add(base / 2);
            p.score = p.score.saturating_add(base / 4);
            b.badges = b.badges.saturating_add(1);
            msg!("Master branch: base={}, a.score={}, p.score={}, total={}", base, a.score, p.score, b.badges);
        } else {
            a.score = a.score.saturating_add(base / 3);
            p.score = p.score.saturating_add((base / 6) + ((mix as u32) & 0x7F));
            b.badges = b.badges.saturating_add(1);
            msg!("Apprentice/Artisan branch: base={}, a.score={}, p.score={}, total={}", base, a.score, p.score, b.badges);
        }

        // 近似平方根で展示指数
        let mut x = (b.badges as u128).max(1);
        let mut i = 0;
        while i < 3 { x = (x + (b.badges as u128 / x)).max(1) / 2; i += 1; }
        l.board = b.key();
        l.exhibit_index = (x as u32).min(1_000_000);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 4 + 4)]
    pub board: Account<'info, Board>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnrollCrafter<'info> {
    #[account(mut)]
    pub board: Account<'info, Board>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 1 + 4)]
    pub crafter: Account<'info, CrafterCard>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 同一親 + 役割不一致で同一口座の二重渡しを不可能化
#[derive(Accounts)]
pub struct MintBadge<'info> {
    #[account(mut)]
    pub board: Account<'info, Board>,
    #[account(mut, has_one = board)]
    pub log: Account<'info, CraftLog>,
    #[account(
        mut,
        has_one = board,
        constraint = actor.role != partner.role @ ErrCode::CosplayBlocked
    )]
    pub actor: Account<'info, CrafterCard>,
    #[account(mut, has_one = board)]
    pub partner: Account<'info, CrafterCard>,
}

#[account]
pub struct Board {
    pub admin: Pubkey,
    pub series: u32,
    pub badges: u32,
}

#[account]
pub struct CrafterCard {
    pub board: Pubkey,
    pub role: CraftRole,
    pub tier: u8,
    pub score: u32,
}

#[account]
pub struct CraftLog {
    pub board: Pubkey,
    pub exhibit_index: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum CraftRole { Apprentice, Artisan, Master }

#[error_code]
pub enum ErrCode { #[msg("Type cosplay blocked in BadgeCraft.")] CosplayBlocked }
