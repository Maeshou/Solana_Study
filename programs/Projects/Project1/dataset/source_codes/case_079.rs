use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfREFLTVL");

#[program]
pub mod referral_rewards {
    use super::*;

    /// ユーザーごとの ReferralAccount を初回のみ初期化
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        let acct = &mut ctx.accounts.referral;
        acct.owner = ctx.accounts.user.key();
        acct.referrals = 0;
        acct.rewards = 0;
        Ok(())
    }

    /// 紹介者(referrer) が新規ユーザー(referee)を紹介して報酬を獲得
    pub fn refer_user(ctx: Context<ReferUser>) -> Result<()> {
        let referrer = &mut ctx.accounts.referrer;
        let referee  = &mut ctx.accounts.referee;
        // 紹介回数インクリメント
        referrer.referrals = referrer.referrals.saturating_add(1);
        // 紹介報酬＋＝固定100
        referrer.rewards  = referrer.rewards.saturating_add(100);
        // 被紹介者ボーナス＋＝固定50
        referee.rewards   = referee.rewards.saturating_add(50);
        msg!(
            "Referrer {:?}: referrals={}, rewards={}",
            referrer.owner, referrer.referrals, referrer.rewards
        );
        msg!(
            "Referee  {:?}: rewards={}",
            referee.owner, referee.rewards
        );
        Ok(())
    }

    /// 自分の紹介情報をログ出力
    pub fn view_referral(ctx: Context<ViewReferral>) -> Result<()> {
        let acct = &ctx.accounts.referral;
        msg!(
            "User {:?}: referrals={}, rewards={}",
            acct.owner, acct.referrals, acct.rewards
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    /// 初回のみ PDA を init
    #[account(
        init,
        payer = user,
        space  = 8 + 32 + 8 + 8,
        seeds = [b"ref", user.key().as_ref()],
        bump
    )]
    pub referral: Account<'info, ReferralAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReferUser<'info> {
    /// 紹介者アカウント
    #[account(
        mut,
        seeds = [b"ref", referrer.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub referrer: Account<'info, ReferralAccount>,
    /// 被紹介者アカウント
    #[account(
        mut,
        seeds = [b"ref", referee.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub referee: Account<'info, ReferralAccount>,

    /// 紹介者自身が署名
    #[account(address = referrer.owner, has_one = owner)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ViewReferral<'info> {
    #[account(
        seeds = [b"ref", user.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub referral: Account<'info, ReferralAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct ReferralAccount {
    /// アカウント所有者
    pub owner: Pubkey,
    /// 紹介回数
    pub referrals: u64,
    /// 獲得報酬ポイント
    pub rewards: u64,
}
