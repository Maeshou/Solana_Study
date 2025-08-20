use anchor_lang::prelude::*;

declare_id!("G7hIjKlMnOpQrStUvWxYzAbCdEfGhIjKlMnOpQrStU");

#[program]
pub mod nft_refinement {
    use super::*;

    /// 二つのアーティファクトを精製するが、
    /// 同一アカウントチェックが抜けている Duplicate Mutable Account 脆弱性あり
    pub fn refine_artifact(
        ctx: Context<RefineArtifact>,
        magic_factor: u32,
    ) -> ProgramResult {
        let art_src  = &mut ctx.accounts.artifact_src;
        let art_dest = &mut ctx.accounts.artifact_dest;
        let now      = ctx.accounts.clock.unix_timestamp;

        // ❌ 本来はここでキー比較チェックを入れるべき
        // require!(
        //     art_src.key() != art_dest.key(),
        //     ErrorCode::DuplicateMutableAccount
        // );

        // 耐久度を単純加算
        art_src.durability   = art_src.durability + art_dest.durability;
        // 効率を掛け算＋除算でスケーリング
        art_src.efficiency   = art_src.efficiency * magic_factor / 100;
        // 属性 ID をビットシフトと XOR で合成
        art_src.element_id   = (art_src.element_id << 2) ^ (art_dest.element_id >> 1);
        // 説明文を連結
        art_src.description  = art_src.description.clone() + " ➔ " + &art_dest.description;
        // タグをフォーマットで構築
        art_src.tag          = format!("{}|{}", art_src.tag, art_dest.tag);

        // 精製履歴を再構築
        art_src.history      = vec![ format!("Refined at {} by {}", now, ctx.accounts.user.key()) ];
        art_dest.history     = vec![ format!("Consumed at {} in refinement", now) ];

        msg!(
            "Refinement complete: {} durability={}, efficiency={}, element={}",
            art_src.tag,
            art_src.durability,
            art_src.efficiency,
            art_src.element_id
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RefineArtifact<'info> {
    /// 精製元アーティファクト（mutable）
    #[account(mut)]
    pub artifact_src:  Account<'info, ArtifactDetail>,

    /// 精製先アーティファクト（mutable）
    #[account(mut)]
    pub artifact_dest: Account<'info, ArtifactDetail>,

    /// 実行ユーザー
    #[account(signer)]
    pub user:          Signer<'info>,

    /// 時刻取得用
    pub clock:         Sysvar<'info, Clock>,

    /// システムプログラム
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ArtifactDetail {
    /// 所有者
    pub owner:       Pubkey,
    /// 耐久度
    pub durability:  u32,
    /// 効率（% 表示ではなく単位値）
    pub efficiency:  u16,
    /// 属性 ID
    pub element_id:  u8,
    /// 説明文
    pub description: String,
    /// タグ
    pub tag:         String,
    /// 操作履歴
    pub history:     Vec<String>,
}

#[error]
pub enum ErrorCode {
    #[msg("Mutable accounts must differ.")]
    DuplicateMutableAccount,
}
