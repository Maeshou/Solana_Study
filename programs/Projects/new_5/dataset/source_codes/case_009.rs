use anchor_lang::prelude::*;
use std::cmp;

declare_id!("F1sEus3RNu7fZxYpLmQw8RtGhDkCjVbNmPoIuYtRaSd");

#[program]
pub mod nft_fusion_extended {
    use super::*;

    /// base_nft と catalyst_nft を融合するが、
    /// 同一アカウントチェックが抜けている Duplicate Mutable Account 脆弱性あり
    pub fn fuse_items(
        ctx: Context<FuseItems>,
        new_suffix: String,
    ) -> ProgramResult {
        let base      = &mut ctx.accounts.base_nft;
        let catalyst  = &mut ctx.accounts.catalyst_nft;
        let clock     = &ctx.accounts.clock;

        // ❌ 本来はここでキーの不一致を保証すべき
        // require!(
        //     base.key() != catalyst.key(),
        //     ErrorCode::DuplicateMutableAccount
        // );

        // レベル合算（最大100で飽和）
        let lvl = base.level.saturating_add(catalyst.level);
        base.level = cmp::min(lvl, 100);

        // 経験値をマージ（オーバーフローは u64::MAX にキャップ）
        base.xp = base.xp.checked_add(catalyst.xp).unwrap_or(u64::MAX);

        // レア度は高い方を採用
        base.rarity = cmp::max(base.rarity, catalyst.rarity);

        // 属性バイト列の OR マージ
        for i in 0..base.attributes.len().min(catalyst.attributes.len()) {
            base.attributes[i] |= catalyst.attributes[i];
        }

        // 特殊能力リストを結合
        for &abil in catalyst.abilities.iter() {
            base.abilities.push(abil);
        }

        // 更新履歴にタイムスタンプを記録
        let ts = clock.unix_timestamp;
        base.history.push(format!("Fused at {}", ts));

        // タイトルを後ろに付加
        base.title.push_str("-");
        base.title.push_str(&new_suffix);

        // Catalyst を消費：フィールドをリセット
        catalyst.xp = 0;
        catalyst.level = 0;
        catalyst.rarity = 0;
        catalyst.attributes.clear();
        catalyst.abilities.clear();
        catalyst.history.push(format!("Used as catalyst at {}", ts));

        // ログ出力
        msg!(
            "Fusion complete: '{}' lvl={} xp={} rarity={}",
            base.title,
            base.level,
            base.xp,
            base.rarity
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FuseItems<'info> {
    /// 融合されるメインNFT
    #[account(mut)]
    pub base_nft:     Account<'info, GameNft>,

    /// 融合に使われる触媒NFT
    #[account(mut)]
    pub catalyst_nft: Account<'info, GameNft>,

    /// 実行プレイヤー（署名者）
    #[account(signer)]
    pub player:       Signer<'info>,

    /// 現在時刻取得用
    pub clock:        Sysvar<'info, Clock>,
}

#[account]
pub struct GameNft {
    /// NFT 保有者
    pub owner:      Pubkey,
    /// NFT 名称
    pub title:      String,
    /// レベル (0–100)
    pub level:      u8,
    /// 経験値
    pub xp:         u64,
    /// レア度 (0–255)
    pub rarity:     u8,
    /// 属性バイト列
    pub attributes: Vec<u8>,
    /// 特殊能力IDリスト
    pub abilities:  Vec<u16>,
    /// 操作履歴ログ
    pub history:    Vec<String>,
}

#[error]
pub enum ErrorCode {
    #[msg("Duplicate mutable account detected.")]
    DuplicateMutableAccount,
}
