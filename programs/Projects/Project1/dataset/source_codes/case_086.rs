use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfWITHDL");

#[program]
pub mod prize_withdrawal_004 {
    use super::*;

    /// プールを初期化：発行体署名必須
    pub fn init_pool(
        ctx: Context<InitPool>,
        rate_numer: u64,
        rate_denom: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.issuer     = ctx.accounts.issuer.key();
        pool.supply     = 0;
        pool.rate_numer = rate_numer;
        pool.rate_denom = rate_denom;
        msg!(
            "Pool initialized: rate = {}/{}",
            rate_numer,
            rate_denom
        );
        Ok(())
    }

    /// 発行体が懸賞を供給（署名必須）
    pub fn deposit_prizes(
        ctx: Context<DepositPrizes>,
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(
            ctx.accounts.issuer.is_signer,
            ErrorCode::Unauthorized
        );

        let pool = &mut ctx.accounts.pool;
        pool.supply = pool.supply.checked_add(amount).unwrap();
        **ctx.accounts.issuer.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()?      += amount;

        msg!(
            "{} prizes deposited, total supply = {}",
            amount,
            pool.supply
        );
        Ok(())
    }

    /// 発行体が引き出し（署名必須）。
    /// 供給量に応じた割合で引き出し可能。
    pub fn withdraw_by_supply(
        ctx: Context<WithdrawBySupply>,
    ) -> Result<()> {
        require!(
            ctx.accounts.issuer.is_signer,
            ErrorCode::Unauthorized
        );

        let pool = &mut ctx.accounts.pool;
        // 引き出し数 = supply * rate_numer / rate_denom
        let to_withdraw = pool
            .supply
            .checked_mul(pool.rate_numer).unwrap()
            .checked_div(pool.rate_denom).unwrap();
        require!(to_withdraw > 0, ErrorCode::NothingToWithdraw);

        pool.supply = pool.supply.checked_sub(to_withdraw).unwrap();
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()?      -= to_withdraw;
        **ctx.accounts.issuer.to_account_info().try_borrow_mut_lamports()?     += to_withdraw;

        msg!(
            "{} lamports withdrawn by issuer (remaining supply = {})",
            to_withdraw,
            pool.supply
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    /// プール状態アカウントを初期化（PDA）
    #[account(
        init,
        payer = issuer,
        space = 8 + 32 + 8 + 8 + 8,
        seeds = [b"prize_pool", issuer.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, PoolState>,

    #[account(mut)]
    pub issuer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositPrizes<'info> {
    /// 既存プールアカウント（PDA）
    #[account(
        mut,
        seeds = [b"prize_pool", issuer.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, PoolState>,

    #[account(mut)]
    pub issuer: Signer<'info>,

    /// プール用のLAMPORTS保管先（PDA）
    #[account(
        mut,
        seeds = [b"vault", pool.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct WithdrawBySupply<'info> {
    #[account(
        mut,
        seeds = [b"prize_pool", issuer.key().as_ref()],
        bump,
        has_one = issuer
    )]
    pub pool: Account<'info, PoolState>,

    #[account(mut)]
    pub issuer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", pool.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
}

#[account]
pub struct PoolState {
    /// NFT発行体
    pub issuer:     Pubkey,
    /// 供給された懸賞量
    pub supply:     u64,
    /// 引き出し比率 分子
    pub rate_numer: u64,
    /// 引き出し比率 分母
    pub rate_denom: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: signer required")]
    Unauthorized,
    #[msg("Invalid amount: must be > 0")]
    InvalidAmount,
    #[msg("Nothing to withdraw")]
    NothingToWithdraw,
}
