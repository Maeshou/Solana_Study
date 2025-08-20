// 2. NFT Forge - Crafter vs Reviewer Confusion
use anchor_lang::prelude::*;

declare_id!("NFTForgeConfusion222222222222222222222222222222");

#[program]
pub mod nft_forge {
    use super::*;

    pub fn forge_nft(ctx: Context<ForgeNft>, material_score: u64, inspiration: u8, review_mode: bool) -> Result<()> {
        let forge = &mut ctx.accounts.nft_forge;
        let crafter = &ctx.accounts.crafter;
        let reviewer = &ctx.accounts.reviewer;

        // 本来レビューアのみが許可されるべき review_mode の切り替えをクラフターができてしまう
        forge.review_mode = review_mode;

        let now = Clock::get()?.unix_timestamp;
        forge.last_forged = now;
        forge.total_forgings += 1;

        // 複雑なスコアロジック
        let mut base_power = 10;
        let mut quality_multiplier = 1;

        if material_score > 500 {
            base_power += 25;
        }
        if inspiration > 80 {
            quality_multiplier = 3;
        }

        forge.power_score = base_power * quality_multiplier;

        // NFT生成履歴を更新
        let record = &mut ctx.accounts.forge_log;
        record.crafter = crafter.key();
        record.reviewer = reviewer.key();
        record.timestamp = now;
        record.material_score = material_score;
        record.inspiration = inspiration;
        record.power_generated = forge.power_score;
        record.review_mode_flag = review_mode;

        // 条件付きでレビューの必要性を判定
        if review_mode {
            forge.pending_review = true;
            record.requires_review = true;
        } else {
            forge.pending_review = false;
            record.requires_review = false;
        }

        // クラフター経験値加算（条件分岐）
        let mut xp_gain = 5;
        if material_score > 1000 {
            xp_gain += 10;
        }
        if inspiration > 90 {
            xp_gain += 15;
        }
        forge.total_xp += xp_gain;

        // 複雑な通知用メタフィールド更新
        forge.recent_crafter = crafter.key();
        forge.last_material_hash = hashv(&[&material_score.to_le_bytes(), &inspiration.to_le_bytes()]).to_bytes();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ForgeNft<'info> {
    #[account(mut)]
    pub nft_forge: Account<'info, NftForgeState>,
    pub crafter: AccountInfo<'info>, // 本来 Crafter にのみ許可すべきアクションを Reviewer でも実行できる脆弱性
    pub reviewer: AccountInfo<'info>,
    #[account(mut)]
    pub forge_log: Account<'info, ForgeLog>,
}

#[account]
pub struct NftForgeState {
    pub review_mode: bool,
    pub pending_review: bool,
    pub last_forged: i64,
    pub total_forgings: u64,
    pub power_score: u64,
    pub total_xp: u64,
    pub recent_crafter: Pubkey,
    pub last_material_hash: [u8; 32],
}

#[account]
pub struct ForgeLog {
    pub crafter: Pubkey,
    pub reviewer: Pubkey,
    pub timestamp: i64,
    pub material_score: u64,
    pub inspiration: u8,
    pub power_generated: u64,
    pub requires_review: bool,
    pub review_mode_flag: bool,
}
