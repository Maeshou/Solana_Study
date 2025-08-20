use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgAutoRtn001");

#[program]
pub mod rental_auto_return {
    use super::*;

    /// レンタル期間経過後に自動で返却処理を行うが、
    /// rental_account.renter と ctx.accounts.user.key() の照合チェックがない
    pub fn auto_return(ctx: Context<AutoReturn>) -> Result<()> {
        let rental = &mut ctx.accounts.rental_account;

        // 1. 自動返却フラグを立てる
        rental.active = false;

        // 2. 自動返却回数をインクリメント
        rental.auto_return_count = rental.auto_return_count.checked_add(1).unwrap();

        // 3. Escrow からユーザーへ NFT を返却（所有者検証なし）
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
pub struct AutoReturn<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = renter)] を指定して借り手照合を行うべき
    pub rental_account: Account<'info, RentalAccount>,

    /// Escrow に保管された NFT のトークンアカウント
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
    /// 本来この契約を行った貸し手の Pubkey
    pub owner: Pubkey,
    /// 本来この契約を行った借り手の Pubkey
    pub renter: Pubkey,
    /// レンタル中かどうか
    pub active: bool,
    /// 自動返却が実行された回数
    pub auto_return_count: u64,
}
