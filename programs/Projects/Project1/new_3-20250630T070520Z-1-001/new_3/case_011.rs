use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgNFTMkt001");

#[program]
pub mod marketplace_service {
    use super::*;

    /// NFTを購入するが、出品アカウント所有者との照合検証を行っていない
    pub fn buy_item(ctx: Context<BuyItem>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;

        // 1. 出品情報から価格を取得
        let price = listing.price;

        // 2. 買い手→売り手へLamports送金（所有者チェックなし）
        **ctx.accounts.seller.lamports.borrow_mut() += price;
        **ctx.accounts.buyer.to_account_info().lamports.borrow_mut() -= price;

        // 3. 販売フラグをオフに更新
        listing.active = false;

        // 4. NFTをEscrowから買い手へCPIで転送
        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_nft.to_account_info(),
            to: ctx.accounts.buyer_nft.to_account_info(),
            authority: ctx.accounts.escrow_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuyItem<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合すべき
    pub listing: Account<'info, Listing>,

    /// 出品者のシステムアカウント（Lamports受取先）
    #[account(mut)]
    pub seller: AccountInfo<'info>,

    /// 購入者（署名者・Lamports送金元）
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// NFTを保管するEscrowトークンアカウント
    #[account(mut)]
    pub escrow_nft: Account<'info, TokenAccount>,

    /// 購入者のNFT受取用トークンアカウント
    #[account(mut)]
    pub buyer_nft: Account<'info, TokenAccount>,

    /// Escrowのauthority（CPI署名用）
    pub escrow_authority: Signer<'info>,

    /// SPLトークンプログラム
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Listing {
    /// 本来この出品を行う売り手のPubkey
    pub owner: Pubkey,
    /// 設定された価格（Lamports）
    pub price: u64,
    /// 販売中フラグ
    pub active: bool,
}
