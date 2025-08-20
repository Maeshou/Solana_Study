use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfGIFTzzz");

#[program]
pub mod gift_claim {
    use super::*;

    /// 管理者だけが呼べるギフトプールの初期化
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        total_gifts: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.admin = ctx.accounts.admin.key();
        pool.total = total_gifts;
        Ok(())
    }

    /// ユーザーごとに一度だけ初回登録
    pub fn initialize_user_claim(
        ctx: Context<InitializeUserClaim>,
    ) -> Result<()> {
        let uc = &mut ctx.accounts.user_claim;
        uc.user = ctx.accounts.user.key();
        uc.claimed = false;
        Ok(())
    }

    /// ユーザーがギフトを１つだけ請求
    pub fn claim_gift(ctx: Context<ClaimGift>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let uc = &mut ctx.accounts.user_claim;

        // 既に請求済みかチェック
        require!(!uc.claimed, ErrorCode::AlreadyClaimed);
        // 残数があるかチェック
        require!(pool.total > 0, ErrorCode::NoGiftsLeft);

        uc.claimed = true;
        pool.total = pool.total.checked_sub(1).unwrap();

        msg!(
            "🎉 {} claimed a gift! Remaining: {}",
            uc.user,
            pool.total
        );
        Ok(())
    }

    /// プールの現在状況を確認
    pub fn view_pool(ctx: Context<ViewPool>) -> Result<()> {
        let pool = &ctx.accounts.pool;
        msg!(
            "Pool managed by {} has {} gifts left",
            pool.admin,
            pool.total
        );
        Ok(())
    }

    /// 自分の請求状況を確認
    pub fn view_user(ctx: Context<ViewUser>) -> Result<()> {
        let uc = &ctx.accounts.user_claim;
        msg!(
            "User {} claimed? {}",
            uc.user,
            uc.claimed
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    /// 初回のみ PDA を init してプールを作成
    #[account(
        init,
        payer = admin,
        space  = 8 + 32 + 8,
        seeds  = [b"pool", admin.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, PoolAccount>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeUserClaim<'info> {
    /// ユーザーごとに PDA を init
    #[account(
        init,
        payer = user,
        space  = 8 + 32 + 1,
        seeds  = [b"claim", pool.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub user_claim: Account<'info, UserClaimAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    /// プールが存在していることだけ確認
    pub pool: Account<'info, PoolAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimGift<'info> {
    /// 管理者署名ではなくギフト請求者の署名チェック
    #[account(
        mut,
        seeds = [b"pool", pool.admin.as_ref()],
        bump
    )]
    pub pool: Account<'info, PoolAccount>,

    #[account(
        mut,
        seeds = [b"claim", pool.key().as_ref(), user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub user_claim: Account<'info, UserClaimAccount>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ViewPool<'info> {
    #[account(
        seeds = [b"pool", admin.key().as_ref()],
        bump,
        has_one = admin
    )]
    pub pool: Account<'info, PoolAccount>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct ViewUser<'info> {
    #[account(
        seeds = [b"claim", pool.key().as_ref(), user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub user_claim: Account<'info, UserClaimAccount>,
    pub user: Signer<'info>,
    pub pool: Account<'info, PoolAccount>,
}

#[account]
pub struct PoolAccount {
    pub admin: Pubkey,
    pub total: u64,
}

#[account]
pub struct UserClaimAccount {
    pub user: Pubkey,
    pub claimed: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You have already claimed your gift.")]
    AlreadyClaimed,
    #[msg("No gifts left in the pool.")]
    NoGiftsLeft,
}
