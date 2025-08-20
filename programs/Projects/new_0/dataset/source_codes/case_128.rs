use anchor_lang::prelude::*;

declare_id!("Movl111111111111111111111111111111111111");

#[program]
pub mod watchlist_manager {
    /// 映画をウォッチリストに追加
    pub fn add_movie(
        ctx: Context<AddMovie>,
        title: String,
        year: u16,
    ) -> Result<()> {
        // タイトル長チェック
        if title.len() > 100 {
            return Err(ErrorCode::TitleTooLong.into());
        }
        // 年代チェック：1888年以前は不可
        if year < 1888 {
            return Err(ErrorCode::InvalidYear.into());
        }
        // 年代チェック：2100年より未来は不可
        if year > 2100 {
            return Err(ErrorCode::InvalidYear.into());
        }

        let movie = &mut ctx.accounts.movie;
        movie.owner   = ctx.accounts.user.key();  // Signer Authorization
        movie.title   = title;
        movie.year    = year;
        movie.watched = false;
        Ok(())
    }

    /// 登録済み映画を「視聴済み」にマーク
    pub fn mark_watched(ctx: Context<MarkWatched>) -> Result<()> {
        let movie = &mut ctx.accounts.movie;
        let user_key = ctx.accounts.user.key();

        // 所有者チェック
        if movie.owner != user_key {
            return Err(ErrorCode::Unauthorized.into());
        }
        // 既に視聴済みなら拒否
        if movie.watched {
            return Err(ErrorCode::AlreadyWatched.into());
        }

        movie.watched = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddMovie<'info> {
    /// 二度の初期化を防止（Reinit Attack）
    #[account(init, payer = user, space = 8 + 32 + 4 + 100 + 2 + 1)]
    pub movie:   Account<'info, MovieAccount>,

    /// 操作を行うユーザー（署名必須）
    #[account(mut)]
    pub user:    Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MarkWatched<'info> {
    /// 型チェック＆所有者チェック（Owner Check / Type Cosplay）
    #[account(mut)]
    pub movie:   Account<'info, MovieAccount>,

    /// 実際に署名したユーザー（Signer Authorization）
    pub user:    Signer<'info>,
}

#[account]
pub struct MovieAccount {
    /// このエントリを操作できるユーザー
    pub owner:   Pubkey,
    /// 映画のタイトル（最大100文字）
    pub title:   String,
    /// 公開年 (u16)
    pub year:    u16,
    /// 視聴済みフラグ
    pub watched: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Title is too long")]
    TitleTooLong,
    #[msg("Year is invalid")]
    InvalidYear,
    #[msg("Already marked as watched")]
    AlreadyWatched,
}
