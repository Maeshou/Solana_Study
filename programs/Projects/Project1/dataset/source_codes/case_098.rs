use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, TokenAccount, Token, Mint};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfCAPEX");

#[program]
pub mod cap_expander {
    use super::*;

    /// 初回呼び出し時にユーザー用データ口座を初期化
    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        let ud = &mut ctx.accounts.user_cap;
        ud.authority = ctx.accounts.user.key();
        ud.cap = 1_000;    // デフォルトの上限
        ud.earned = 0;
        msg!("Initialized cap={} for {}", ud.cap, ud.authority);
        Ok(())
    }

    /// 通常のアーニング処理。earned+amount が cap 以下であることを保証
    pub fn earn(ctx: Context<Earn>, amount: u64) -> Result<()> {
        require!(ctx.accounts.user.is_signer, ErrorCode::Unauthorized);

        let ud = &mut ctx.accounts.user_cap;
        let new_earned = ud.earned.checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;
        require!(new_earned <= ud.cap, ErrorCode::CapExceeded);

        ud.earned = new_earned;
        msg!("Earned {} (total {}) / cap {}", amount, ud.earned, ud.cap);
        Ok(())
    }

    /// トークンをバーンして cap を amount × 10 だけ拡張
    pub fn burn_to_extend(ctx: Context<BurnExtend>, amount: u64) -> Result<()> {
        require!(ctx.accounts.user.is_signer, ErrorCode::Unauthorized);

        // ユーザーのトークン口座からバーン
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint:      ctx.accounts.mint.to_account_info(),
                from:      ctx.accounts.user_token.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::burn(cpi_ctx, amount)?;

        // cap を拡張（倍率は 10 倍）
        let ud = &mut ctx.accounts.user_cap;
        let add = amount.checked_mul(10).ok_or(ErrorCode::Overflow)?;
        ud.cap = ud.cap.checked_add(add).ok_or(ErrorCode::Overflow)?;
        msg!("Burned {} tokens, cap extended by {} → {}", amount, add, ud.cap);

        Ok(())
    }
}

/// 初期化用
#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 8,
        seeds = [b"user_cap", user.key().as_ref()],
        bump
    )]
    pub user_cap: Account<'info, UserCap>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// アーニング用
#[derive(Accounts)]
pub struct Earn<'info> {
    #[account(
        mut,
        seeds = [b"user_cap", user.key().as_ref()],
        bump,
        has_one = authority @ ErrorCode::Unauthorized
    )]
    pub user_cap: Account<'info, UserCap>,

    /// Authority = 同じく user
    pub authority: SystemAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
}

/// バーン→cap拡張用
#[derive(Accounts)]
pub struct BurnExtend<'info> {
    #[account(
        mut,
        seeds = [b"user_cap", user.key().as_ref()],
        bump,
        has_one = authority @ ErrorCode::Unauthorized
    )]
    pub user_cap: Account<'info, UserCap>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// ユーザーの保有するトークン口座
    #[account(
        mut,
        constraint = user_token.owner == user.key(),
        constraint = user_token.mint == mint.key()
    )]
    pub user_token: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct UserCap {
    pub authority: Pubkey,
    pub cap:       u64,
    pub earned:    u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Signature required")]
    Unauthorized,
    #[msg("Cap would overflow")]
    Overflow,
    #[msg("Earning exceeds cap")]
    CapExceeded,
}
