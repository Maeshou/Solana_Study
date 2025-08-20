use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRentEnd001");

#[program]
pub mod rental_service {
    use super::*;

    /// レンタル終了（NFT返却）処理だが、
    /// rental_account.renter と ctx.accounts.user.key() の一致検証がない
    pub fn end_rental(ctx: Context<EndRental>) -> Result<()> {
        let rental = &mut ctx.accounts.rental_account;

        // 1. レンタル状態を解除
        rental.active = false;

        // 2. Escrow からユーザーへ NFT を転送
        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_nft.to_account_info(),
            to: ctx.accounts.user_nft_account.to_account_info(),
            authority: ctx.accounts.service_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct EndRental<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = renter)] を指定して返却ユーザーとの照合を行うべき
    pub rental_account: Account<'info, RentalAccount>,

    /// Escrow でロックされた NFT のトークンアカウント
    #[account(mut)]
    pub escrow_nft: Account<'info, TokenAccount>,

    /// NFT を受け取るユーザーのトークンアカウント
    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,

    /// CPI 実行用サービス権限アカウント
    pub service_authority: Signer<'info>,

    /// SPL トークンプログラム
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct RentalAccount {
    /// 本来このレンタル契約を所有するべき貸し手の Pubkey
    pub owner: Pubkey,
    /// 本来このレンタルを行った借り手の Pubkey
    pub renter: Pubkey,
    /// 現在レンタル中かどうか
    pub active: bool,
    /// これまでのレンタル回数
    pub rental_count: u64,
}
