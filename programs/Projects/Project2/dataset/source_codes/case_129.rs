use anchor_lang::prelude::*;

declare_id!("Favr111111111111111111111111111111111111");

#[program]
pub mod favorites_manager {
    /// アイテムをお気に入りに追加
    pub fn add_favorite(
        ctx: Context<AddFavorite>,
        item: Pubkey,
    ) -> Result<()> {
        let fav = &mut ctx.accounts.favorite;
        // 既にお気に入りなら拒否
        if fav.is_active {
            return Err(ErrorCode::AlreadyFavorited.into());
        }
        fav.owner     = ctx.accounts.user.key();  // Signer Authorization
        fav.item      = item;
        fav.is_active = true;
        Ok(())
    }

    /// お気に入りを解除
    pub fn remove_favorite(ctx: Context<ModifyFavorite>) -> Result<()> {
        let fav = &mut ctx.accounts.favorite;
        // 所有者チェック
        if fav.owner != ctx.accounts.user.key() {
            return Err(ErrorCode::Unauthorized.into());
        }
        // すでに解除済みなら拒否
        if !fav.is_active {
            return Err(ErrorCode::AlreadyRemoved.into());
        }
        fav.is_active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddFavorite<'info> {
    /// 同一アカウント再初期化防止（Reinit Attack）
    #[account(init, payer = user, space = 8 + 32 + 32 + 1)]
    pub favorite: Account<'info, Favorite>,

    /// 操作するユーザー（署名必須）
    #[account(mut)]
    pub user:     Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyFavorite<'info> {
    /// 型チェック＆Owner Check
    #[account(mut)]
    pub favorite: Account<'info, Favorite>,

    /// 操作するユーザー（署名必須）
    pub user:     Signer<'info>,
}

#[account]
pub struct Favorite {
    /// このお気に入りを操作できるユーザー
    pub owner:     Pubkey,
    /// お気に入り対象のアイテム
    pub item:      Pubkey,
    /// アクティブ（true）／解除済み（false）
    pub is_active: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")]
    Unauthorized,
    #[msg("既にお気に入りに登録されています")]
    AlreadyFavorited,
    #[msg("既に解除されています")]
    AlreadyRemoved,
}
