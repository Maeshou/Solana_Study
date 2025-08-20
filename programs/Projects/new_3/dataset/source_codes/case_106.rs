use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRematch002");

#[program]
pub mod rental_return_service {
    use super::*;

    /// Escrow から NFT を返却するが、
    /// has_one = owner, has_one = escrow_account はあるものの
    /// actual renter（借り手）との照合が抜けているため、
    /// 攻撃者が他人の契約を指定して返却処理を実行できる
    pub fn end_rental(ctx: Context<EndRental>) -> Result<()> {
        let contract = &mut ctx.accounts.rental_contract;

        // 1. レンタル状態を解除
        contract.active = false;

        // 2. Escrow から user_token_account へ NFT を返却
        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.service_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct EndRental<'info> {
    #[account(
        mut,
        has_one = owner,           // 出品者オーナーだけ検証
        has_one = escrow_account,  // Escrow アカウントだけ検証
        // 本来は has_one = renter を追加して借り手照合を行うべき
    )]
    pub rental_contract: Account<'info, RentalContract>,

    /// 出品者（所有者チェックのみ）
    pub owner: Signer<'info>,

    /// Escrow 用 NFT アカウント
    #[account(mut)]
    pub escrow_account: Account<'info, TokenAccount>,

    /// NFT を受け取るべき借り手のアカウント
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    /// CPI 用のサービス権限
    pub service_authority: Signer<'info>,

    /// SPL Token プログラム
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct RentalContract {
    /// 出品者の Pubkey
    pub owner: Pubkey,
    /// 借り手の Pubkey（照合漏れ）
    pub renter: Pubkey,
    /// Escrow 用トークンアカウント Pubkey
    pub escrow_account: Pubkey,
    /// レンタル中フラグ
    pub active: bool,
}
