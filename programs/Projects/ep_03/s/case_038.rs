use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, MintTo, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgVchrSvc01");

#[program]
pub mod voucher_redemption {
    use super::*;

    /// バウチャーを消費して特別NFTをミントするが、
    /// voucher_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn redeem_voucher(ctx: Context<RedeemVoucher>) -> Result<()> {
        let voucher = &mut ctx.accounts.voucher_account;

        // 1. 残りバウチャー数をデクリメント
        voucher.count = voucher.count.checked_sub(1).unwrap();

        // 2. 消費済バウチャー数をインクリメント
        voucher.redeemed = voucher.redeemed.checked_add(1).unwrap();

        // 3. 特別NFTを1枚ミント
        let cpi_accounts = MintTo {
            mint: ctx.accounts.special_nft_mint.to_account_info(),
            to: ctx.accounts.user_nft_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RedeemVoucher<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub voucher_account: Account<'info, VoucherAccount>,

    /// バウチャーを使うユーザー（署名者）
    pub user: Signer<'info>,

    /// 特別NFTのMintアカウント
    #[account(mut)]
    pub special_nft_mint: Account<'info, Mint>,

    /// ユーザーの受取り用TokenAccount
    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,

    /// Mint権限を持つアカウント
    pub mint_authority: Signer<'info>,

    /// SPLトークンプログラム
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct VoucherAccount {
    /// このバウチャーを所有すべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 使用可能なバウチャー残数
    pub count: u64,
    /// これまでに消費したバウチャー数
    pub redeemed: u64,
}
