use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct SessionManager(pub u8, pub Vec<(Pubkey, i64)>); // (bump, Vec<(user, expires_ts)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzV8");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of sessions reached")]
    MaxSessionsReached,
    #[msg("Session not found")]
    SessionNotFound,
}

#[program]
pub mod session_manager {
    use super::*;

    const MAX_SESSIONS: usize = 32;

    /// 初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bump = *ctx.bumps.get("manager").unwrap();
        ctx.accounts.manager.0 = bump;
        Ok(())
    }

    /// セッション開始：件数制限チェック＋expires_ts で追加
    pub fn start_session(
        ctx: Context<Modify>, 
        user: Pubkey, 
        expires_ts: i64
    ) -> Result<()> {
        let list = &mut ctx.accounts.manager.1;
        if list.len() >= MAX_SESSIONS {
            return err!(ErrorCode::MaxSessionsReached);
        }
        list.push((user, expires_ts));
        Ok(())
    }

    /// セッション検証：期限切れでなければログ出力
    pub fn validate_session(ctx: Context<Modify>, user: Pubkey) -> Result<()> {
        let list = &ctx.accounts.manager.1;
        let now  = ctx.accounts.clock.unix_timestamp;
        for &(u, exp) in list.iter() {
            if u == user {
                if exp >= now {
                    msg!("Session for {} is valid", u);
                } else {
                    msg!("Session for {} has expired", u);
                }
            }
        }
        Ok(())
    }

    /// 期限切れセッションの一括消去
    pub fn purge_expired(ctx: Context<Modify>) -> Result<()> {
        let now  = ctx.accounts.clock.unix_timestamp;
        ctx.accounts.manager.1.retain(|&(_, exp)| {
            if exp >= now {
                true
            } else {
                false
            }
        });
        Ok(())
    }

    /// アクティブセッション数をログ出力
    pub fn count_active(ctx: Context<Modify>) -> Result<()> {
        let now   = ctx.accounts.clock.unix_timestamp;
        let mut cnt = 0u64;
        for &(_, exp) in ctx.accounts.manager.1.iter() {
            if exp >= now {
                cnt = cnt.wrapping_add(1);
            }
        }
        msg!("Active sessions: {}", cnt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"manager", authority.key().as_ref()],
        bump,
        // discriminator(8)+bump(1)+Vec len(4)+max32*(32+8)
        space = 8 + 1 + 4 + 32 * (32 + 8)
    )]
    pub manager:   Account<'info, SessionManager>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"manager", authority.key().as_ref()],
        bump = manager.0
    )]
    pub manager:   Account<'info, SessionManager>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
