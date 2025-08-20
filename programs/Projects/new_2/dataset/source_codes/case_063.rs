use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Transfer, transfer};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqAuctionSettleV1");

#[program]
pub mod nft_auction_settle {
    use super::*;

    /// オークションを決済し、入札金を売り手へ払い出しつつ
    /// NFT を落札者へ転送する  
    /// （`settle_nft_account` の owner チェックを行っていないため、
    ///  攻撃者が自分のトークンアカウントを指定するだけで
    ///  他人の落札アイテムを横取りできます）
    pub fn settle_auction(ctx: Context<SettleAuction>) -> Result<()> {
        // 1) auction_account の先頭8バイトから価格を読み取る
        let raw = ctx.accounts.auction_account.data.borrow();
        if raw.len() < 8 {
            return err!(ErrorCode::DataTooShort);
        }
        let mut price_bytes = [0u8; 8];
        price_bytes.copy_from_slice(&raw[..8]);
        let price = u64::from_le_bytes(price_bytes);

        // 2) lamports を売り手へ払い出し
        **ctx.accounts.seller.to_account_info().lamports.borrow_mut() += price;
        **ctx.accounts.auction_funds.to_account_info().lamports.borrow_mut() -= price;

        // 3) NFT を escrow から落札者のアカウントへ転送
        //    ★ settle_nft_account の owner チェックを省略！
        let cpi_accounts = Transfer {
            from:      ctx.accounts.escrow_nft_account.to_account_info(),
            to:        ctx.accounts.settle_nft_account.to_account_info(),
            authority: ctx.accounts.auction_owner.to_account_info(),
        };
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
            ),
            1,
        )?;

        msg!(
            "Auction settled: paid {} lamports to {}, NFT delivered to {}",
            price,
            ctx.accounts.seller.key(),
            ctx.accounts.settle_nft_account.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SettleAuction<'info> {
    /// CHECK: owner == program_id の検証を全く行っていない生の AccountInfo
    #[account(mut)]
    pub auction_account:    AccountInfo<'info>,

    /// lamports 保管用 Escrow アカウント（owner チェックなし）
    #[account(mut)]
    pub auction_funds:      AccountInfo<'info>,

    /// 売り手の lamports 受取先（owner チェックなし）
    #[account(mut)]
    pub seller:             AccountInfo<'info>,

    /// NFT エスクロー用 TokenAccount（owner チェックなし）
    #[account(mut)]
    pub escrow_nft_account: AccountInfo<'info>,

    /// NFT 転送先 TokenAccount（owner チェックを省略）
    #[account(mut)]
    pub settle_nft_account: AccountInfo<'info>,

    /// CPI 実行用署名者（通常はオークションの預託者）
    pub auction_owner:      Signer<'info>,

    /// SPL Token プログラム CPI 用
    pub token_program:      Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが短すぎて価格を読み取れません")]
    DataTooShort,
}
