use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Wishl1111111111111111111111111111111111");

#[program]
pub mod wishlist_manager {
    /// ウィッシュリストの初期化
    pub fn init_list(ctx: Context<InitList>) -> Result<()> {
        let list = &mut ctx.accounts.wishlist;
        list.owner = ctx.accounts.user.key();
        list.entries.clear();
        Ok(())
    }

    /// アイテムをリストに追加
    pub fn add_entry(ctx: Context<ModifyList>, description: String) -> Result<()> {
        let list = &mut ctx.accounts.wishlist;
        let now  = ctx.accounts.clock.unix_timestamp;

        // 権限と入力チェック
        require!(list.owner == ctx.accounts.user.key(), ErrorCode::Unauthorized);
        require!(list.entries.len() < 30, ErrorCode::ListFull);
        require!(description.len() <= 64, ErrorCode::TooLong);

        // 重複チェック
        let mut exists = false;
        for entry in list.entries.iter() {
            if entry.description == description {
                exists = true;
                break;
            }
        }
        require!(!exists, ErrorCode::Duplicate);

        // 追加
        list.entries.push(WishEntry {
            description,
            added_at: now,
            purchased: false,
            purchased_at: 0,
        });
        Ok(())
    }

    /// アイテムを購入済みにマーク
    pub fn purchase(ctx: Context<ModifyList>, index: u32) -> Result<()> {
        let list = &mut ctx.accounts.wishlist;
        let idx  = index as usize;
        let now  = ctx.accounts.clock.unix_timestamp;

        require!(list.owner == ctx.accounts.user.key(), ErrorCode::Unauthorized);
        require!(idx < list.entries.len(), ErrorCode::IndexOutOfBounds);
        require!(!list.entries[idx].purchased, ErrorCode::AlreadyPurchased);

        let item = &mut list.entries[idx];
        item.purchased = true;
        item.purchased_at = now;
        Ok(())
    }

    /// アイテムをリストから削除
    pub fn remove_entry(ctx: Context<ModifyList>, index: u32) -> Result<()> {
        let list = &mut ctx.accounts.wishlist;
        let idx  = index as usize;

        require!(list.owner == ctx.accounts.user.key(), ErrorCode::Unauthorized);
        require!(idx < list.entries.len(), ErrorCode::IndexOutOfBounds);

        list.entries.remove(idx);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitList<'info> {
    #[account(init, payer = user, space = 8 + 32 + 4 + (30 * (4 + 64 + 8 + 1 + 8)))]
    pub wishlist:       Account<'info, Wishlist>,
    #[account(mut)]
    pub user:           Signer<'info>,
    pub clock:          Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyList<'info> {
    #[account(mut)]
    pub wishlist: Account<'info, Wishlist>,
    pub user:     Signer<'info>,
    pub clock:    Sysvar<'info, Clock>,
}

#[account]
pub struct Wishlist {
    /// リスト所有者
    pub owner:   Pubkey,
    /// ウィッシュリスト項目
    pub entries: Vec<WishEntry>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WishEntry {
    /// アイテム説明
    pub description: String,
    /// 追加時刻 (UNIX)
    pub added_at:    i64,
    /// 購入済みフラグ
    pub purchased:   bool,
    /// 購入時刻 (UNIX)
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
