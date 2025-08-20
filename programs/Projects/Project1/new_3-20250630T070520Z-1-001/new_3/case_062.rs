use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgLeaseSvc02");

#[program]
pub mod lease_service {
    use super::*;

    /// レースの終了（NFT返却）処理だが、
    /// lease_account.lessee と ctx.accounts.user.key() の照合検証がない
    pub fn end_lease(ctx: Context<EndLease>) -> Result<()> {
        let lease = &mut ctx.accounts.lease_account;

        // 1. リース状態を解除
        lease.active = false;

        // 2. Escrow からユーザーへ NFT を返却
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
pub struct EndLease<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = lessee)] を指定して返却ユーザーとの照合を行うべき
    pub lease_account: Account<'info, LeaseAccount>,

    /// Escrow に預けられた NFT 用のトークンアカウント
    #[account(mut)]
    pub escrow_nft: Account<'info, TokenAccount>,

    /// NFT を受け取るユーザーのトークンアカウント
    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,

    /// CPI 実行権限を持つサービス権限アカウント
    pub service_authority: Signer<'info>,

    /// SPL トークンプログラム
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct LeaseAccount {
    /// 本来この契約を締結した貸し手の Pubkey
    pub owner: Pubkey,
    /// 本来この契約を締結した借り手の Pubkey
    pub lessee: Pubkey,
    /// 現在のリース状態 (true=貸出中)
    pub active: bool,
    /// 終了処理を実行した回数
    pub end_count: u64,
}
