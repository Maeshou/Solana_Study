use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Wishl1111111111111111111111111111111111");

const MAX_ITEMS: usize = 30;

#[program]
pub mod wishlist_manager {
    /// 新しいウィッシュリストを作成
    pub fn init_list(ctx: Context<InitList>) -> Result<()> {
        let list = &mut ctx.accounts.wishlist;
        list.owner = ctx.accounts.user.key();
        list.entries = Vec::new();
        Ok(())
    }

    /// アイテムを追加
    pub fn add_entry(ctx: Context<ModifyList>, description: String) -> Result<()> {
        let list = &mut ctx.accounts.wishlist;
        let now = ctx.accounts.clock.unix_timestamp;

        require!(list.owner == ctx.accounts.user.key(), ErrorCode::Unauthorized);
        require!(list.entries.len() < MAX_ITEMS, ErrorCode::ListFull);
        require!(description.len() <= 64, ErrorCode::TooLong);

        // 重複チェック
        for e in list.entries.iter() {
            if e.description == description {
                return Err(ErrorCode::Duplicate.into());
            }
        }

        list.entries.push(WishEntry { description, added_at: now, purchased: false, purchased_at: 0 });
        Ok(())
    }

    /// アイテムを購入済みにマーク
    pub fn purchase(ctx: Context<ModifyList>, index: u32) -> Result<()> {
        let list = &mut ctx.accounts.wishlist;
        let idx = index as usize;
        let now = ctx.accounts.clock.unix_timestamp;

        require!(list.owner == ctx.accounts.user.key(), ErrorCode::Unauthorized);
        require!(idx < list.entries.len(), ErrorCode::IndexOutOfBounds);
        require!(!list.entries[idx].purchased, ErrorCode::AlreadyPurchased);

        // 真ブランチで複数更新
        let entry = &mut list.entries[idx];
        entry.purchased = true;
        entry.purchased_at = now;
        Ok(())
    }

    /// アイテムをリストから削除
    pub fn remove_entry(ctx: Context<ModifyList>, index: u32) -> Result<()> {
        let list = &mut ctx.accounts.wishlist;
        let idx = index as usize;

        require!(list.owner == ctx.accounts.user.key(), ErrorCode::Unauthorized);
        require!(idx < list.entries.len(), ErrorCode::IndexOutOfBounds);

        list.entries.remove(idx);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitList<'info> {
    #[account(init, payer = user, space = 8 + 32 + 4 + (MAX_ITEMS * (4 + 64 + 8 + 1 + 8)))]
    pub wishlist: Account<'info, Wishlist>,
    #[account(mut)] pub user: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyList<'info> {
    #[account(mut)] pub wishlist: Account<'info, Wishlist>,
    pub user: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct Wishlist {
    pub owner: Pubkey,
    pub entries: Vec<WishEntry>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WishEntry {
    pub description: String,
    pub added_at: i64,
    pub purchased: bool,
    pub purchased_at: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")] Unauthorized,
    #[msg("リストが上限に達しました")] ListFull,
    #[msg("説明が長すぎます")] TooLong,
    #[msg("既に追加済みです")] Duplicate,
    #[msg("インデックスが範囲外です")] IndexOutOfBounds,
    #[msg("既に購入済みです")] AlreadyPurchased,
}
