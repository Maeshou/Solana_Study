// 6. メタ情報＋ヒストリー
use anchor_lang::prelude::*;

declare_id!("Met66666666666666666666666666666666");

#[program]
pub mod reinit_metadata_v2 {
    use super::*;

    // メタ情報を読み込む
    pub fn load_metadata(
        ctx: Context<LoadMetadata>,
        url: String,
    ) -> Result<()> {
        let meta = &mut ctx.accounts.metadata;
        meta.url = url;
        meta.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    // バージョンを増やす
    pub fn bump_meta(
        ctx: Context<LoadMetadata>,
    ) -> Result<()> {
        let meta = &mut ctx.accounts.metadata;
        meta.version = meta.version + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LoadMetadata<'info> {
    #[account(mut)]
    pub metadata: Account<'info, MetadataData>,
    /// CHECK: 履歴格納用、逐次上書き
    #[account(mut)]
    pub history: AccountInfo<'info>,
}

#[account]
pub struct MetadataData {
    pub url: String,
    pub version: u8,
    pub updated_at: i64,
}
