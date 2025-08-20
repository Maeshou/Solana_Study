// (1) Emblem Foundry — 紋章工房と鑑定カード
use anchor_lang::prelude::*;
declare_id!("EmB1eMFoundryA1111111111111111111111111111");

#[program]
pub mod emblem_foundry {
    use super::*;
    use EmblemRole::*;

    pub fn init_foundry(ctx: Context<InitFoundry>, name: String) -> Result<()> {
        let f = &mut ctx.accounts.foundry;
        f.owner = ctx.accounts.master.key();
        f.name = name;
        f.batch = 0;
        f.mix = 0;
        Ok(())
    }

    pub fn init_emblem(ctx: Context<InitEmblem>, role: EmblemRole) -> Result<()> {
        let e = &mut ctx.accounts.emblem;
        e.foundry = ctx.accounts.foundry.key();
        e.role = role;
        e.quality = 0;
        e.hist = [0; 8];
        Ok(())
    }

    pub fn forge(ctx: Context<Forge>, salt: u64) -> Result<()> {
        let f = &mut ctx.accounts.foundry;
        let s = &mut ctx.accounts.smith;
        let a = &mut ctx.accounts.appraiser;
        let l = &mut ctx.accounts.ledger;

        // ループ：履歴と擬似混合
        let mut h = salt ^ (f.batch as u64);
        for i in 0..8 {
            h = h.rotate_left(9) ^ 0x9E3779B97F4A7C15u64;
            s.hist[i] = s.hist[i].saturating_add(((h >> (i * 5)) & 0xFF) as u32);
            f.mix = f.mix ^ ((h as u32) & 0xFFFF);
        }

        // 分岐：Smith か否かでロジック分岐（各4行以上）
        if s.role == Smith {
            s.quality = s.quality.saturating_add(((h & 0xFFFF) as u32) + 7);
            f.batch = f.batch.saturating_add(1);
            l.lines = l.lines.saturating_add(1);
            l.last = l.last.wrapping_add(h.rotate_left(7));
            msg!("Smith path: forged new emblem");
        } else {
            a.quality = a.quality.saturating_add((((h >> 3) & 0x7FFF) as u32) + 5);
            f.mix = f.mix.rotate_right(3) ^ ((h as u32) & 0xFFFF);
            l.lines = l.lines.saturating_add(2);
            l.last = l.last ^ h.rotate_right(11);
            msg!("Non-smith path: appraiser evaluated");
        }
        Ok(())
    }
}

// ───────── Accounts ─────────

#[derive(Accounts)]
pub struct InitFoundry<'info> {
    #[account(init, payer = master, space = 8 + Foundry::MAX)]
    pub foundry: Account<'info, Foundry>,
    #[account(mut)]
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitEmblem<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub foundry: Account<'info, Foundry>,
    #[account(init, payer = user, space = 8 + EmblemCard::MAX)]
    pub emblem: Account<'info, EmblemCard>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// 明示的なプログラムID固定（多層フェンスの一部）
    #[account(address = crate::ID)]
    pub program_id_alias: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Forge<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub foundry: Account<'info, Foundry>,

    #[account(mut, has_one = foundry, owner = crate::ID)]
    pub ledger: Account<'info, ForgeLedger>,

    #[account(mut, has_one = foundry, owner = crate::ID)]
    pub smith: Account<'info, EmblemCard>,

    #[account(
        mut,
        has_one = foundry,
        owner = crate::ID,
        // 一意フィールド（role）の不一致で同一Pubkey二重渡しを禁止
        constraint = smith.role != appraiser.role @ ErrCode::CosplayBlocked
    )]
    pub appraiser: Account<'info, EmblemCard>,

    pub owner: Signer<'info>,
}

// ───────── Data ─────────

#[account]
pub struct Foundry {
    pub owner: Pubkey,
    pub name: String,
    pub batch: u64,
    pub mix: u32,
}
impl Foundry {
    pub const MAX: usize = 32 + 4 + 64 + 8 + 4;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum EmblemRole {
    Smith,
    Appraiser,
    Archivist,
}
use EmblemRole::*;

#[account]
pub struct EmblemCard {
    pub foundry: Pubkey,
    pub role: EmblemRole,
    pub quality: u32,
    pub hist: [u32; 8],
}
impl EmblemCard {
    pub const MAX: usize = 32 + 1 + 4 + 4 * 8;
}

#[account]
pub struct ForgeLedger {
    pub foundry: Pubkey,
    pub last: u64,
    pub lines: u32,
}
impl ForgeLedger {
    pub const MAX: usize = 32 + 8 + 4;
}

#[error_code]
pub enum ErrCode {
    #[msg("Type Cosplay blocked by role mismatch")]
    CosplayBlocked,
}
