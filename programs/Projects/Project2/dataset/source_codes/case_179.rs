use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct ChatRoom(pub u8, pub Vec<(i64, Pubkey, String)>);

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzV1");

#[program]
pub mod chat_room {
    use super::*;

    /// ChatRoom 初期化：bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bump = *ctx.bumps.get("chat_room").unwrap();
        ctx.accounts.room.0 = bump;
        Ok(())
    }

    /// メッセージ投稿：現在時刻・署名者・本文を追加
    pub fn post_message(
        ctx: Context<ModifyRoom>,
        content: String,
    ) -> Result<()> {
        let list = &mut ctx.accounts.room.1;
        let now  = ctx.accounts.clock.unix_timestamp;
        list.push((now, ctx.accounts.user.key(), content));
        Ok(())
    }

    /// 古いメッセージ削除：threshold_ts より古いものを取り除く
    pub fn purge_old(
        ctx: Context<ModifyRoom>,
        threshold_ts: i64,
    ) -> Result<()> {
        let list = &mut ctx.accounts.room.1;
        list.retain(|&(ts, _, _)| ts > threshold_ts);
        Ok(())
    }

    /// 最近のメッセージ数報告：now から window_secs 以内の数をログ出力
    pub fn count_recent(
        ctx: Context<ModifyRoom>,
        window_secs: i64,
    ) -> Result<()> {
        let list = &ctx.accounts.room.1;
        let now  = ctx.accounts.clock.unix_timestamp;
        let mut cnt = 0u64;
        for &(ts, _, _) in list.iter() {
            if ts + window_secs >= now {
                cnt = cnt.wrapping_add(1);
            }
        }
        msg!("Recent messages: {}", cnt);
        Ok(())
    }
}

// ── Context 定義は末尾に配置 ──
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"chat_room", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec len(4) + max10*(8+32+4+200)
        space = 8 + 1 + 4 + 10 * (8 + 32 + 4 + 200)
    )]
    pub room:       Account<'info, ChatRoom>,
    #[account(mut)]
    pub authority:  Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyRoom<'info> {
    #[account(
        mut,
        seeds = [b"chat_room", authority.key().as_ref()],
        bump = room.0,
    )]
    pub room:      Account<'info, ChatRoom>,
    /// 投稿者（署名必須）
    #[account(signer)]
    pub user:      AccountInfo<'info>,
    /// 時刻取得用
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
