use anchor_lang::prelude::*;

declare_id!("Book111111111111111111111111111111111111");

#[program]
pub mod bookmark_manager {
    /// ブックマーク作成
    pub fn create_bookmark(
        ctx: Context<CreateBookmark>,
        url: String,
        title: String,
    ) -> Result<()> {
        // 入力長チェック（オーバーフロー／memory issues 防止）
        require!(url.len() <= 128, ErrorCode::UrlTooLong);
        require!(title.len() <= 64, ErrorCode::TitleTooLong);

        let bm = &mut ctx.accounts.bookmark;
        // Signer Authorization & Owner Check
        bm.owner = ctx.accounts.user.key();
        bm.url   = url;
        bm.title = title;
        Ok(())
    }

    /// ブックマーク更新
    pub fn update_bookmark(
        ctx: Context<UpdateBookmark>,
        new_url: String,
        new_title: String,
    ) -> Result<()> {
        let bm = &mut ctx.accounts.bookmark;
        // Account Matching + Signer Authorization
        require!(
            bm.owner == ctx.accounts.user.key(),
            ErrorCode::Unauthorized
        );
        // 入力長チェック
        require!(new_url.len() <= 128, ErrorCode::UrlTooLong);
        require!(new_title.len() <= 64, ErrorCode::TitleTooLong);

        bm.url   = new_url;
        bm.title = new_title;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateBookmark<'info> {
    /// init 制約で同一アカウント再初期化防止（Reinit Attack）
    #[account(init, payer = user, space = 8 + 32 + 4 + 128 + 4 + 64)]
    pub bookmark: Account<'info, Bookmark>,

    /// ブックマーク所有者（署名必須）
    #[account(mut)]
    pub user:     Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateBookmark<'info> {
    /// Anchor の Account<> による Owner Check／Type Cosplay
    #[account(mut)]
    pub bookmark: Account<'info, Bookmark>,

    /// 実際に署名したユーザー（Signer Authorization）
    pub user:     Signer<'info>,
}

#[account]
pub struct Bookmark {
    /// このブックマークを操作できるユーザー
    pub owner: Pubkey,
    /// URL（最大128文字）
    pub url:   String,
    /// タイトル（最大64文字）
    pub title: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("URL is too long")]
    UrlTooLong,
    #[msg("Title is too long")]
    TitleTooLong,
}
