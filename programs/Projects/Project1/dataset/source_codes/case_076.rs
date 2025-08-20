use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfBADGE01");

#[program]
pub mod unique_badge {
    use super::*;

    /// バッジを PDA アカウントとして初期化し、名前を保存します。
    pub fn initialize_badge(ctx: Context<InitializeBadge>, name: String) -> Result<()> {
        let badge = &mut ctx.accounts.badge;
        badge.owner = ctx.accounts.user.key();
        // 名前は最大 32 バイトまで保存
        badge.name_bytes = name.as_bytes()[..name.as_bytes().len().min(32)]
            .try_into()
            .unwrap_or([0; 32]);
        badge.redeemed = false;
        badge.redeemed_ts = 0;
        Ok(())
    }

    /// まだ未引換のバッジを「引換済み」にマークし、タイムスタンプを記録します。
    pub fn redeem_badge(ctx: Context<RedeemBadge>) -> Result<()> {
        let badge = &mut ctx.accounts.badge;
        // bool → u8 に変換してから saturating_sub で未引換フラグチェック
        let flag = badge.redeemed as u8;
        let can = 1u8.saturating_sub(flag);         // 1→未引換、0→すでに引換
        // redeemed を更新 (can == 1 の場合のみ true になる)
        badge.redeemed = can == 1;
        // 現在時刻を取得し、can==1 のときのみ有効化
        let now = Clock::get()?.unix_timestamp;
        badge.redeemed_ts = now.saturating_mul(can as i64);
        msg!(
            "Badge '{}' redeemed at {} (was redeemed={}).",
            String::from_utf8_lossy(&badge.name_bytes),
            badge.redeemed_ts,
            badge.redeemed,
        );
        Ok(())
    }

    /// バッジの状態をログ出力します。
    pub fn view_badge(ctx: Context<ViewBadge>) -> Result<()> {
        let badge = &ctx.accounts.badge;
        msg!("Owner       : {:?}", badge.owner);
        msg!("Name        : {}", String::from_utf8_lossy(&badge.name_bytes));
        msg!("Redeemed    : {}", badge.redeemed);
        msg!("Redeemed TS : {}", badge.redeemed_ts);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeBadge<'info> {
    /// 初回のみ PDA アカウントを作成・初期化
    #[account(
        init,
        payer = user,
        space  = 8 + 32 + 32 + 1 + 8,
        seeds = [b"badge", user.key().as_ref()],
        bump
    )]
    pub badge: Account<'info, Badge>,

    /// 操作にはユーザーの署名が必要
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RedeemBadge<'info> {
    /// PDA と所有者チェックで不正アクセス防止
    #[account(
        seeds = [b"badge", user.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub badge: Account<'info, Badge>,

    /// 操作にはユーザーの署名が必要
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ViewBadge<'info> {
    /// 誰でも自分のバッジを見ることが可能
    #[account(
        seeds = [b"badge", user.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub badge: Account<'info, Badge>,

    pub user: Signer<'info>,
}

#[account]
pub struct Badge {
    /// バッジ所有者
    pub owner: Pubkey,
    /// バッジ名（最大32バイト、残りは0埋め）
    pub name_bytes: [u8; 32],
    /// 引換済みフラグ
    pub redeemed: bool,
    /// 引換時刻（Unix epoch）
    pub redeemed_ts: i64,
}
