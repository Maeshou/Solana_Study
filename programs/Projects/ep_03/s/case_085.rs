use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgMigrn001");

#[program]
pub mod nft_migration_service {
    use super::*;

    /// NFT のコレクション情報を更新するが、
    /// metadata_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn migrate_collection(
        ctx: Context<ModifyMetadata>,
        new_collection: Pubkey,
    ) -> Result<()> {
        let meta = &mut ctx.accounts.metadata_account;
        apply_migration(meta, new_collection);
        Ok(())
    }

    /// NFT のバージョンをインクリメントするが、
    /// metadata_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn bump_version(ctx: Context<ModifyMetadata>) -> Result<()> {
        let meta = &mut ctx.accounts.metadata_account;
        increment_version(meta);
        Ok(())
    }
}

/// コレクションフィールドを書き換えてマイグレーション回数を増やすヘルパー
fn apply_migration(meta: &mut MetadataAccount, new_coll: Pubkey) {
    meta.collection = new_coll;
    meta.migrations = meta.migrations.checked_add(1).unwrap();
}

/// バージョンを単純にインクリメントするヘルパー
fn increment_version(meta: &mut MetadataAccount) {
    meta.version = meta.version.checked_add(1).unwrap();
}

#[derive(Accounts)]
pub struct ModifyMetadata<'info> {
    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して所有者照合を行うべき
    pub metadata_account: Account<'info, MetadataAccount>,
    /// 操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct MetadataAccount {
    /// 本来このメタデータを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// NFT が所属するコレクションのミントアドレス
    pub collection: Pubkey,
    /// 現在のメタデータバージョン
    pub version: u8,
    /// これまでにマイグレーションした回数
    pub migrations: u64,
}
