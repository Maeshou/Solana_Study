use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Transfer, CpiContext, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgEscrowSvc02");

#[program]
pub mod escrow_service {
    use super::*;

    /// エスクローを成立させ、トークンを交換するが、
    /// has_one = initializer, has_one = temp_token_account のみ検証され、
    /// 本来必要な has_one = initializer_token_account が抜けているため
    /// 攻撃者が他人の初期トークンアカウントを指定して不正に引き出せる
    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        let escrow = &ctx.accounts.escrow_account;
        let amount = escrow.expected_amount;

        // Escrow から初期化ユーザーへトークンを返却
        let cpi_accounts = Transfer {
            from: ctx.accounts.temp_token_account.to_account_info(),
            to: ctx.accounts.initializer_token_account.to_account_info(),
            authority: ctx.accounts.initializer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(
        mut,
        has_one = initializer,
        has_one = temp_token_account,
        // 本来は has_one = initializer_token_account も指定して
        // escrow_account.initializer_token_account と一致検証すべき
    )]
    pub escrow_account: Account<'info, EscrowAccount>,

    /// エスクローを初期化したユーザー（署名者）
    #[account(mut)]
    pub initializer: Signer<'info>,

    /// 一時的にトークンを預ける TokenAccount
    #[account(mut)]
    pub temp_token_account: Account<'info, TokenAccount>,

    /// 初期化ユーザーのトークン受取用 TokenAccount
    #[account(mut)]
    pub initializer_token_account: Account<'info, TokenAccount>,

    /// SPL Token プログラム
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct EscrowAccount {
    /// 初期化ユーザーの Pubkey
    pub initializer: Pubkey,
    /// 一時的にロックされたトークンアカウントの Pubkey
    pub temp_token_account: Pubkey,
    /// 初期化ユーザーの受取先トークンアカウントの Pubkey
    pub initializer_token_account: Pubkey,
    /// 交換予定トークン量
    pub expected_amount: u64,
}
