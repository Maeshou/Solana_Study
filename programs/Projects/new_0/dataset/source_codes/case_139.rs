use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("TagM111111111111111111111111111111111111");

const MAX_TAGS: usize = 20;

#[program]
pub mod tag_manager {
    /// タグリストの初期化
    pub fn init_tag_list(ctx: Context<InitTagList>) -> Result<()> {
        let tags = &mut ctx.accounts.tag_list;
        tags.owner = ctx.accounts.user.key();
        tags.tags = Vec::new();
        Ok(())
    }

    /// 新しいタグを追加
    pub fn create_tag(ctx: Context<ModifyTagList>, name: String) -> Result<()> {
        let tags = &mut ctx.accounts.tag_list;
        let now = ctx.accounts.clock.unix_timestamp;

        // 権限チェック + 入力検証
        require!(tags.owner == ctx.accounts.user.key(), ErrorCode::Unauthorized);
        require!(tags.tags.len() < MAX_TAGS, ErrorCode::MaxTagsReached);
        require!(name.len() <= 32, ErrorCode::NameTooLong);

        // 重複チェック
        let mut duplicate = false;
        for t in tags.tags.iter() {
            if t.name == name {
                duplicate = true;
                break;
            }
        }
        require!(!duplicate, ErrorCode::DuplicateTag);

        // 真ブランチで複数の状態変化
        tags.tags.push(TagItem { name, created_at: now });
        Ok(())
    }

    /// タグの名前を変更
    pub fn rename_tag(ctx: Context<ModifyTagList>, index: u32, new_name: String) -> Result<()> {
        let tags = &mut ctx.accounts.tag_list;
        let user = ctx.accounts.user.key();

        require!(tags.owner == user, ErrorCode::Unauthorized);
        require!((index as usize) < tags.tags.len(), ErrorCode::IndexOutOfBounds);
        require!(new_name.len() <= 32, ErrorCode::NameTooLong);

        // 同名タグの存在チェック
        let mut exists = false;
        for t in tags.tags.iter() {
            if t.name == new_name {
                exists = true;
                break;
            }
        }
        require!(!exists, ErrorCode::DuplicateTag);

        tags.tags[index as usize].name = new_name;
        Ok(())
    }

    /// タグを削除
    pub fn delete_tag(ctx: Context<ModifyTagList>, index: u32) -> Result<()> {
        let tags = &mut ctx.accounts.tag_list;

        require!(tags.owner == ctx.accounts.user.key(), ErrorCode::Unauthorized);
        let idx = tags.tags.iter().enumerate()
            .find_map(|(i, _)| if i == index as usize { Some(i) } else { None });
        let i = idx.ok_or(ErrorCode::IndexOutOfBounds)?;
        tags.tags.remove(i);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTagList<'info> {
    #[account(init, payer = user, space = 8 + 32 + 4 + (MAX_TAGS * (4 + 32 + 8)))]
    pub tag_list:      Account<'info, TagList>,
    #[account(mut)] pub user: Sysvar<'info, Signer<'info>>,
    pub clock:         Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyTagList<'info> {
    #[account(mut)] pub tag_list: Account<'info, TagList>,
    pub user: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct TagList {
    pub owner: Pubkey,
    pub tags: Vec<TagItem>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TagItem {
    pub name: String,
    pub created_at: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")] Unauthorized,
    #[msg("タグ数の上限に達しました")] MaxTagsReached,
    #[msg("タグ名が長すぎます")] NameTooLong,
    #[msg("同名のタグが既に存在します")] DuplicateTag,
    #[msg("インデックスが範囲外です")] IndexOutOfBounds,
}
