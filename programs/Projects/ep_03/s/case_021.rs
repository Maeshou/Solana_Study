use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRentSvc001");

#[program]
pub mod rental_service {
    use super::*;

    /// NFTを一定期間レンタルするが、所有者照合チェックがない
    pub fn rent_nft(ctx: Context<RentNFT>) -> Result<()> {
        let rent_acc = &mut ctx.accounts.rental_account;
        let fee = ctx.accounts.config.fee;

        // 1. レンタル料を徴収（user→treasury）
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() -= fee;
        **ctx.accounts.treasury.to_account_info().lamports.borrow_mut() += fee;

        // 2. VaultからユーザーへNFTを転送
        let cpi_accounts = Transfer {
            from: ctx.accounts.owner_nft_account.to_account_info(),
            to: ctx.accounts.user_nft_account.to_account_info(),
            authority: ctx.accounts.service_authority.to_account_info(),
        };
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
            1,
        )?;

        // 3. レンタル情報を更新
        rent_acc.renter = ctx.accounts.user.key();
        rent_acc.active = true;
        rent_acc.rental_count = rent_acc.rental_count.checked_add(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RentNFT<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub rental_account: Account<'info, RentalAccount>,

    /// レンタル実行者（署名者）
    pub user: Signer<'info>,

    /// レンタル料受取口座
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    /// 所有者のNFT保管アカウント
    #[account(mut)]
    pub owner_nft_account: Account<'info, TokenAccount>,

    /// レンタルユーザーのNFT受取アカウント
    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,

    /// CPI実行権限を持つサービス権限アカウント
    pub service_authority: Signer<'info>,

    /// SPLトークンプログラム
    pub token_program: Program<'info, Token>,

    /// レンタル料設定アカウント
    pub config: Account<'info, RentalConfig>,
}

#[account]
pub struct RentalAccount {
    /// 本来このレンタルを所有するユーザーのPubkey
    pub owner: Pubkey,
    /// 現在レンタル中のユーザーPubkey
    pub renter: Pubkey,
    /// レンタル状態（true=レンタル中）
    pub active: bool,
    /// これまでのレンタル回数
    pub rental_count: u64,
}

#[account]
pub struct RentalConfig {
    /// 1回あたりのレンタル料（Lamports）
    pub fee: u64,
}
