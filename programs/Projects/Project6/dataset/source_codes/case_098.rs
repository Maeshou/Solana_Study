// (1) Relic Vault — 遺物保管庫とキー/監査カード
use anchor_lang::prelude::*;
declare_id!("Re1icVau1tA777770000000000000000000000001");

#[program]
pub mod relic_vault {
    use super::*;
    use Tier::*;

    pub fn init_vault(ctx: Context<InitVault>, label: String, nonce: u32) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        v.owner = ctx.accounts.curator.key();

        // ラベルを64文字に制限しつつ指紋化
        let mut name = label;
        if name.len() > 64 { name.truncate(64); }
        v.label = name;

        // 指紋＋ローリング初期化
        let mut fp: u32 = 0x811C9DC5 ^ nonce;
        for (i, b) in v.label.as_bytes().iter().enumerate() {
            fp = fp.rotate_left((i as u32) & 7) ^ (*b as u32);
            fp = fp.wrapping_mul(0x01000193);
        }
        v.fingerprint = fp;
        let mut s = (nonce as u64) ^ 0x9E3779B97F4A7C15;
        for i in 0..v.noise.len() {
            s = s.rotate_left(9) ^ (fp as u64);
            v.noise[i] = (s as u32) ^ fp.rotate_right((i as u32) & 31);
        }

        v.epoch = ((nonce % 251) + 1) as u16;
        v.ledger_mix = 0;
        Ok(())
    }

    pub fn init_keycard(ctx: Context<InitKeyCard>, tier: Tier, channel: u8, seed: u64) -> Result<()> {
        let k = &mut ctx.accounts.keycard;
        k.vault = ctx.accounts.vault.key();
        k.tier = tier;
        k.channel = channel; // 一意フィールド
        k.power = 0;
        let mut s = seed ^ ((channel as u64) << 17);
        for i in 0..k.hist.len() {
            s = s.rotate_left(7) ^ 0xC3A5C85C97CB3127u64;
            k.hist[i] = ((s >> (i * 6)) as u32) & 0x3FFF;
        }
        Ok(())
    }

    pub fn rotate_audit(ctx: Context<RotateAudit>, delta: u16, hint: u32) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        let kc = &mut ctx.accounts.keycard;
        let ac = &mut ctx.accounts.auditcard;
        let l = &mut ctx.accounts.log;

        // ループ：簡易平方根近似で尺度導出
        let mut x = (v.ledger_mix as u64 + hint as u64).max(1);
        let n = x;
        for _ in 0..6 { x = (x + n / x) >> 1; }
        let scale = (x as u32).min(65_535);

        if kc.tier == Tier1 {
            // if ブロック（4行以上）
            kc.power = kc.power.saturating_add((scale & 0x3FF) + delta as u32);
            v.ledger_mix = v.ledger_mix.rotate_left(3) ^ ((kc.channel as u32) << 4);
            l.lines = l.lines.saturating_add(1);
            l.hash = l.hash.wrapping_add((scale as u64).rotate_left(11));
            msg!("Tier1 path: keycard power boosted, vault mixed");
        } else {
            // else ブロック（4行以上）
            ac.audit = ac.audit.saturating_add(((scale >> 2) & 0x1FF) + (delta as u32 / 2));
            v.ledger_mix = v.ledger_mix.rotate_right(5) ^ ((ac.channel as u32) << 6);
            l.lines = l.lines.saturating_add(2);
            l.hash = l.hash ^ ((scale as u64).rotate_right(9));
            msg!("Non-Tier1 path: auditcard updated, vault mixed differently");
        }

        // 追加ループ：ノイズ更新
        for i in 0..v.noise.len() {
            v.noise[i] = v.noise[i].rotate_left((i as u32) & 7) ^ (l.hash as u32 & 0xFFFF);
        }
        Ok(())
    }
}

// ───────── Accounts ─────────

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(init, payer = curator, space = 8 + Vault::MAX)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub curator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitKeyCard<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub vault: Account<'info, Vault>,
    #[account(init, payer = operator, space = 8 + KeyCard::MAX)]
    pub keycard: Account<'info, KeyCard>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RotateAudit<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub vault: Account<'info, Vault>,
    #[account(mut, has_one = vault, owner = crate::ID)]
    pub log: Account<'info, VaultLog>,
    #[account(mut, has_one = vault, owner = crate::ID)]
    pub keycard: Account<'info, KeyCard>,
    #[account(
        mut,
        has_one = vault,
        owner = crate::ID,
        // 一意フィールド（channel）不一致で同一口座の二重渡しを遮断
        constraint = keycard.channel != auditcard.channel @ ErrCode::CosplayBlocked
    )]
    pub auditcard: Account<'info, AuditCard>,
    pub owner: Signer<'info>,
}

// ───────── Data ─────────

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub label: String,
    pub epoch: u16,
    pub fingerprint: u32,
    pub ledger_mix: u32,
    pub noise: [u32; 6],
}
impl Vault {
    pub const MAX: usize = 32 + 4 + 64 + 2 + 4 + 4 + 4 * 6;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Tier { Tier1, Tier2, Tier3 }
use Tier::*;

#[account]
pub struct KeyCard {
    pub vault: Pubkey,
    pub tier: Tier,
    pub channel: u8,   // 一意フィールド
    pub power: u32,
    pub hist: [u32; 8],
}
impl KeyCard {
    pub const MAX: usize = 32 + 1 + 1 + 4 + 4 * 8;
}

#[account]
pub struct AuditCard {
    pub vault: Pubkey,
    pub channel: u8,   // 一意フィールド（KeyCardと衝突回避）
    pub audit: u32,
}
impl AuditCard {
    pub const MAX: usize = 32 + 1 + 4;
}

#[account]
pub struct VaultLog {
    pub vault: Pubkey,
    pub hash: u64,
    pub lines: u32,
}
impl VaultLog {
    pub const MAX: usize = 32 + 8 + 4;
}

#[error_code]
pub enum ErrCode {
    #[msg("Type Cosplay blocked by unique field mismatch")]
    CosplayBlocked,
}
