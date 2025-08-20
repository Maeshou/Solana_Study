use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSwapRematch02");

#[program]
pub mod swap_service_rematch {
    use super::*;

    /// TokenA と TokenB を交換するが、
    /// SwapAccount に has_one = pool は指定しているものの、
    /// スワップを申し込んだユーザー（owner）との照合がないため、
    /// 攻撃者が他人のアカウントを使って任意のプールでスワップ可能
    pub fn swap(ctx: Context<Swap>, amount_in: u64) -> Result<()> {
        let acct = &mut ctx.accounts.swap_account;
        // 累計スワップ回数を記録
        acct.swaps = acct.swaps.saturating_add(1);

        // 1. TokenA をユーザーからプールへ移動
        let cpi_in = Transfer {
            from: ctx.accounts.user_source.to_account_info(),
            to: ctx.accounts.pool_source.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx_in = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_in);
        token::transfer(cpi_ctx_in, amount_in)?;

        // 2. 手数料を差し引いた分の TokenB をプールからユーザーへ移動
        let amount_out = amount_in.checked_sub(ctx.accounts.config.fee).unwrap();
        let cpi_out = Transfer {
            from: ctx.accounts.pool_dest.to_account_info(),
            to: ctx.accounts.user_dest.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info(),
        };
        let cpi_ctx_out = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_out);
        token::transfer(cpi_ctx_out, amount_out)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut, has_one = pool)]
    /// has_one = pool はあるが、
    /// 本来は has_one = owner を追加してスワップ実行ユーザーを検証すべき
    pub swap_account: Account<'info, SwapAccount>,

    /// 関連する流動性プール
    pub pool: Account<'info, PoolAccount>,

    /// ユーザーの TokenA アカウント
    #[account(mut)]
    pub user_source: Account<'info, TokenAccount>,

    /// プールの TokenA アカウント
    #[account(mut)]
    pub pool_source: Account<'info, TokenAccount>,

    /// プールの TokenB アカウント
    #[account(mut)]
    pub pool_dest: Account<'info, TokenAccount>,

    /// ユーザーの TokenB アカウント
    #[account(mut)]
    pub user_dest: Account<'info, TokenAccount>,

    /// プール操作権限（Mint/Burn などに使う）
    pub pool_authority: Signer<'info>,

    /// スワップ手数料設定
    pub config: Account<'info, SwapConfig>,

    /// SPL Token プログラム
    pub token_program: Program<'info, Token>,

    /// スワップを実行するユーザー（署名者）
    pub user: Signer<'info>,  // swap_account.owner 照合が抜けている
}

#[account]
pub struct SwapAccount {
    /// 本来このスワップを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 紐づく PoolAccount の Pubkey
    pub pool: Pubkey,
    /// 実行されたスワップ回数
    pub swaps: u64,
}

#[account]
pub struct PoolAccount {
    /// TokenA Vault のアドレス
    pub token_a_vault: Pubkey,
    /// TokenB Vault のアドレス
    pub token_b_vault: Pubkey,
    /// 紐づく SwapAccount のアドレス
    pub swap_account: Pubkey,
}

#[account]
pub struct SwapConfig {
    /// 1 回あたりのスワップ手数料 (TokenA 単位)
    pub fee: u64,
}
