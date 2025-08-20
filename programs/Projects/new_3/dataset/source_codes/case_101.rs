use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRematch001");

#[program]
pub mod rental_return_service {
    use super::*;

    /// レンタル契約を終了し、Escrow から NFT を返却するが、
    /// has_one を使っているものの `rental_account.renter` との一致検証がなく、
    /// 攻撃者が別のユーザーの返却処理をトリガーできる
    pub fn end_rental(ctx: Context<EndRental>) -> Result<()> {
        let rental = &mut ctx.accounts.rental_account;

        // 1. アクティブフラグを解除
        rental.active = false;

        // 2. Escrow から user_account へ NFT を返却
        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_account.to_account_info(),
            to: ctx.accounts.user_account.to_account_info(),
            authority: ctx.accounts.service_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct EndRental<'info> {
    #[account(mut, has_one = owner, has_one = escrow_account)]
    /// 本来は `has_one = renter` も指定して借り手照合を行うべき
    pub rental_account: Account<'info, RentalContract>,

    /// 出品者（所有者チェックのみ）
    pub owner: Signer<'info>,

    /// Escrow に預けられた NFT のトークンアカウント
    #[account(mut)]
    pub escrow_account: Account<'info, TokenAccount>,

    /// NFT を受け取るべき借り手のトークンアカウント
    #[account(mut)]
    pub user_account: Account<'info, TokenAccount>,

    /// CPI 実行用サービス権限
    pub service_authority: Signer<'info>,

    /// SPL Token プログラム
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct RentalContract {
    /// 出品者の Pubkey
    pub owner: Pubkey,
    /// 借り手の Pubkey（検証漏れ）
    pub renter: Pubkey,
    /// Escrow 用トークンアカウントの Pubkey
    pub escrow_account: Pubkey,
    /// レンタル中かどうか
    pub active: bool,
}
