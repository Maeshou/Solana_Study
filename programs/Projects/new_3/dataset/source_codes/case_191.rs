use anchor_lang::prelude::*;
declare_id!("PlaylstVuln111111111111111111111111111111");

/// 音楽プレイリスト
#[account]
pub struct Playlist {
    pub owner:    Pubkey,      // プレイリスト作成者
    pub name:     String,      // プレイリスト名
    pub tracks:   Vec<String>, // 曲 URI 一覧
}

/// 曲追加記録
#[account]
pub struct TrackRecord {
    pub user:        Pubkey,   // 追加を行ったユーザー
    pub playlist:    Pubkey,   // 本来は Playlist.key() と一致すべき
    pub track_uri:   String,   // 追加された曲の URI
}

#[derive(Accounts)]
pub struct CreatePlaylist<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 64 + 4 + (4 + 128) * 50)]
    pub playlist:    Account<'info, Playlist>,
    #[account(mut)]
    pub owner:       Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddTrack<'info> {
    /// Playlist.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub playlist:    Account<'info, Playlist>,

    /// TrackRecord.playlist ⇔ playlist.key() の検証がないため、
    /// 偽の TrackRecord を渡しても通過してしまう
    #[account(init, payer = user, space = 8 + 32 + 32 + 4 + 128)]
    pub record:      Account<'info, TrackRecord>,

    #[account(mut)]
    pub owner:       Signer<'info>,
    #[account(mut)]
    pub user:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveTrack<'info> {
    /// TrackRecord.user == user.key() は検証される
    #[account(mut, has_one = user)]
    pub record:      Account<'info, TrackRecord>,

    /// Playlist.key() ⇔ record.playlist の検証がないため、
    /// 偽物の TrackRecord で別のプレイリストから曲を取り除ける
    #[account(mut)]
    pub playlist:    Account<'info, Playlist>,

    pub user:        Signer<'info>,
}

#[program]
pub mod playlist_vuln {
    use super::*;

    /// 新しいプレイリストを作成
    pub fn create_playlist(ctx: Context<CreatePlaylist>, name: String) -> Result<()> {
        let pl = &mut ctx.accounts.playlist;
        pl.owner  = ctx.accounts.owner.key();
        pl.name   = name;
        // tracks は init 時に空 Vec
        Ok(())
    }

    /// 曲を追加
    pub fn add_track(ctx: Context<AddTrack>, uri: String) -> Result<()> {
        let pl = &mut ctx.accounts.playlist;
        let r  = &mut ctx.accounts.record;

        // 脆弱性ポイント:
        // r.playlist = pl.key(); の検証がない
        r.user      = ctx.accounts.user.key();
        r.playlist  = pl.key();
        r.track_uri = uri.clone();

        // Vec::push で曲 URI を追加
        pl.tracks.push(uri);
        Ok(())
    }

    /// 曲を取り除き
    pub fn remove_track(ctx: Context<RemoveTrack>) -> Result<()> {
        let pl = &mut ctx.accounts.playlist;

        // 本来必要:
        // require_keys_eq!(ctx.accounts.record.playlist, pl.key(), ErrorCode::PlaylistMismatch);

        // Vec::pop で最後に追加された曲を削除（分岐・ループなし）
        pl.tracks.pop();
        Ok(())
    }
}
