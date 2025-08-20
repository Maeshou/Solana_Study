use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUF");

#[program]
pub mod playlist_manager {
    use super::*;

    /// プレイリスト作成：ID・名前を受け取り、曲数を 0 で初期化
    pub fn create_playlist(
        ctx: Context<CreatePlaylist>,
        bump: u8,
        playlist_id: u64,
        name: String,
    ) -> Result<()> {
        *ctx.accounts.playlist = Playlist {
            owner:         ctx.accounts.user.key(),
            bump,
            playlist_id,
            name,
            tracks_count:  0,
        };
        Ok(())
    }

    /// 曲追加：tracks_count をインクリメント
    pub fn add_track(ctx: Context<TrackAction>) -> Result<()> {
        let pl = &mut ctx.accounts.playlist;
        pl.tracks_count = pl.tracks_count.wrapping_add(1);
        Ok(())
    }

    /// 曲削除：tracks_count をデクリメント
    pub fn remove_track(ctx: Context<TrackAction>) -> Result<()> {
        let pl = &mut ctx.accounts.playlist;
        pl.tracks_count = pl.tracks_count.wrapping_sub(1);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, playlist_id: u64)]
pub struct CreatePlaylist<'info> {
    /// PDA で生成する Playlist アカウント
    #[account(
        init,
        payer = user,
        // 8 (discriminator) + 32 (owner) + 1 (bump) + 8 (playlist_id)
        // + 4 + name 最大100バイト + 8 (tracks_count)
        space = 8 + 32 + 1 + 8 + 4 + 100 + 8,
        seeds = [b"playlist", user.key().as_ref(), &playlist_id.to_le_bytes()],
        bump
    )]
    pub playlist: Account<'info, Playlist>,

    /// プレイリスト作成者（署名必須）
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TrackAction<'info> {
    /// 既存の Playlist（PDA／bump 検証 + オーナーチェック）
    #[account(
        mut,
        seeds = [b"playlist", owner.key().as_ref(), &playlist.playlist_id.to_le_bytes()],
        bump = playlist.bump,
        has_one = owner
    )]
    pub playlist: Account<'info, Playlist>,

    /// プレイリスト所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Playlist {
    pub owner:        Pubkey,
    pub bump:         u8,
    pub playlist_id:  u64,
    pub name:         String,
    pub tracks_count: u64,
}
