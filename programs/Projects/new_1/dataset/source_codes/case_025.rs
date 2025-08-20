use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxREFERRAL0000000000000");

#[program]
pub mod referral_program {
    use super::*;

    /// ユーザーが友人 (friend) を紹介すると、
    /// - referrer_data.count を ++  
    /// - referrer_data.points に bonus を加算  
    /// - friend_data.points に bonus/2 を加算  
    /// 署名チェックは user／friend ともに省略しています。
    pub fn refer_friend(ctx: Context<ReferCtx>, bonus: u64) {
        let now = ctx.accounts.clock.unix_timestamp;

        // リファラー情報更新
        let r = &mut ctx.accounts.referrer_data;
        r.count       = r.count.checked_add(1).unwrap();
        r.points      = r.points.checked_add(bonus).unwrap();
        r.last_time   = now;

        // 友人情報更新
        let f = &mut ctx.accounts.friend_data;
        f.points      = f.points.checked_add(bonus / 2).unwrap();
        f.joined      = true;
        f.joined_at   = now;

        emit!(ReferralEvent {
            referrer:    *ctx.accounts.user.key,
            friend:      *ctx.accounts.friend.key,
            count:       r.count,
            ref_points:  r.points,
            friend_points: f.points,
        });
    }
}

#[derive(Accounts)]
pub struct ReferCtx<'info> {
    /// トランザクション手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:      Signer<'info>,

    /// 紹介者ユーザー（署名チェック omitted intentionally）
    pub user:           AccountInfo<'info>,

    /// リファラー情報を保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        space     = 8 + 8 + 8 + 8,
        seeds     = [b"referrer", user.key().as_ref()],
        bump
    )]
    pub referrer_data:  Account<'info, ReferrerData>,

    /// 友人ユーザー（署名チェック omitted intentionally）
    pub friend:         AccountInfo<'info>,

    /// 友人情報を保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        space     = 8 + 8 + 1 + 8,
        seeds     = [b"friend", friend.key().as_ref()],
        bump
    )]
    pub friend_data:    Account<'info, FriendData>,

    pub clock:          Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct ReferrerData {
    pub count:      u64, // 紹介回数
    pub points:     u64, // 紹介ポイント累計
    pub last_time:  i64, // 最後の紹介時刻
}

#[account]
pub struct FriendData {
    pub points:     u64,  // 受け取った招待ボーナス
    pub joined:     bool, // 招待を受け取ったか
    pub joined_at:  i64,  // 招待を受けた時刻
}

#[event]
pub struct ReferralEvent {
    pub referrer:      Pubkey,
    pub friend:        Pubkey,
    pub count:         u64,
    pub ref_points:    u64,
    pub friend_points: u64,
}
