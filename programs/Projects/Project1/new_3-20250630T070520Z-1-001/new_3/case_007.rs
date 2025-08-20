use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgGiftSvc0001");

#[program]
pub mod nft_gift_service {
    use super::*;

    /// ギフト手数料を支払い、NFTを別ユーザーへ送信する処理
    pub fn send_gift(ctx: Context<SendGift>) -> Result<()> {
        let gift_acc = &mut ctx.accounts.gift_account;

        // 1. 設定アカウントからギフト手数料を取得
        let fee = ctx.accounts.config.gift_fee;

        // 2. ユーザーの通貨アカウントから手数料をサービス権限の口座へ転送
        let payment_accounts = Transfer {
            from: ctx.accounts.user_currency_account.to_account_info(),
            to: ctx.accounts.fee_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), payment_accounts),
            fee,
        )?;

        // 3. ギフト回数をインクリメント
        gift_acc.gifts_count = gift_acc.gifts_count.checked_add(1).unwrap();

        // 4. Vault に保管された NFT を受取ユーザーのアカウントへ転送
        let nft_transfer_accounts = Transfer {
            from: ctx.accounts.vault_nft_account.to_account_info(),
            to: ctx.accounts.recipient_nft_account.to_account_info(),
            authority: ctx.accounts.service_authority.to_account_info(),
        };
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), nft_transfer_accounts),
            1,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendGift<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub gift_account: Account<'info, GiftAccount>,

    /// ギフト処理を実行するユーザー
    pub user: Signer<'info>,

    /// ユーザーの通貨アカウント（手数料支払い元）
    #[account(mut)]
    pub user_currency_account: Account<'info, TokenAccount>,

    /// サービス権限で集約する手数料受取口座
    #[account(mut)]
    pub fee_vault: Account<'info, TokenAccount>,

    /// NFTを保管するVaultアカウント
    #[account(mut)]
    pub vault_nft_account: Account<'info, TokenAccount>,

    /// ギフト先ユーザーのNFT受取用アカウント
    #[account(mut)]
    pub recipient_nft_account: Account<'info, TokenAccount>,

    /// CPI実行権限を持つサービス権限アカウント
    pub service_authority: Signer<'info>,

    /// SPLトークンプログラム
    pub token_program: Program<'info, Token>,

    /// ギフト手数料を保持する設定アカウント
    pub config: Account<'info, GiftConfig>,
}

#[account]
pub struct GiftAccount {
    /// 本来このギフト機能を所有するユーザーのPubkey
    pub owner: Pubkey,
    /// これまでに送信したギフトの回数
    pub gifts_count: u64,
}

#[account]
pub struct GiftConfig {
    /// ギフト1回あたりの手数料（トークン単位）
    pub gift_fee: u64,
}
