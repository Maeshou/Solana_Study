use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct Wishlist(pub u8, pub Vec<(u64, i64)>); // (bump, Vec<(item_id, added_ts)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVB");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of items reached")]
    MaxItemsReached,
    #[msg("Item not found")]
    ItemNotFound,
}

#[program]
pub mod wishlist_manager {
    use super::*;

    const MAX_ITEMS: usize = 20;

    /// リスト初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let b = *ctx.bumps.get("wishlist").unwrap();
        ctx.accounts.list.0 = b;
        Ok(())
    }

    /// アイテム追加：件数制限チェック＋timestamp 付きで追加
    pub fn add_item(ctx: Context<Modify>, item_id: u64) -> Result<()> {
        let entries = &mut ctx.accounts.list.1;
        if entries.len() >= MAX_ITEMS {
            return err!(ErrorCode::MaxItemsReached);
        }
        let now = ctx.accounts.clock.unix_timestamp;
        entries.push((item_id, now));
        Ok(())
    }

    /// アイテム削除：該当項目を一括除去
    pub fn remove_item(ctx: Context<Modify>, item_id: u64) -> Result<()> {
        let entries = &mut ctx.accounts.list.1;
        entries.retain(|&(id, _)| {
            if id == item_id {
                false
            } else {
                true
            }
        });
        Ok(())
    }

    /// 古いアイテム削除：added_ts が cutoff より小さいものを除去
    pub fn purge_old(ctx: Context<Modify>, cutoff: i64) -> Result<()> {
        let entries = &mut ctx.accounts.list.1;
        entries.retain(|&(_, ts)| {
            if ts < cutoff {
                false
            } else {
                true
            }
        });
        Ok(())
    }

    /// 現在のアイテム数をログ出力
    pub fn count_items(ctx: Context<Modify>) -> Result<()> {
        let cnt = ctx.accounts.list.1.len() as u64;
        msg!("Wishlist count: {}", cnt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"wishlist", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec len(4) + max20*(8+8)
        space = 8 + 1 + 4 + 20 * (8 + 8)
    )]
    pub list:      Account<'info, Wishlist>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"wishlist", authority.key().as_ref()],
        bump = list.0,
    )]
    pub list:      Account<'info, Wishlist>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
