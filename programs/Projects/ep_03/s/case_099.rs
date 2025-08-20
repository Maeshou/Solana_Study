use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgGiftSvc003");

#[program]
pub mod gift_service {
    use super::*;

    /// NFT をギフトし、手数料を徴収するが、
    /// gift_account.owner と ctx.accounts.sender.key() の照合チェックがない
    pub fn send_gift(ctx: Context<SendGift>, note: String) -> Result<()> {
        let gift_acc = &mut ctx.accounts.gift_account;
        let fee = ctx.accounts.config.gift_fee;

        // 1. ギフト手数料を徴収（所有者検証なし）
        **ctx.accounts.sender.to_account_info().lamports.borrow_mut() -= fee;
        **ctx.accounts.fee_pool.to_account_info().lamports.borrow_mut() += fee;

        // 2. ギフト送信回数とメモを記録
        gift_acc.gifts_sent = gift_acc.gifts_sent.saturating_add(1);
        gift_acc.last_note = note;

        // 3. NFT を送信者から受取人へ転送
        let cpi_accounts = Transfer {
            from: ctx.accounts.sender_nft.to_account_info(),
            to: ctx.accounts.recipient_nft.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendGift<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付けて
    /// gift_account.owner と sender.key() を照合すべき
    pub gift_account: Account<'info, GiftAccount>,

    /// 徴収した手数料を貯めるプール
    #[account(mut)]
    pub fee_pool: AccountInfo<'info>,

    /// ギフトを送るユーザー（署名者）
    #[account(mut)]
    pub sender: Signer<'info>,

    /// 送信者の NFT トークンアカウント
    #[account(mut)]
    pub sender_nft: Account<'info, TokenAccount>,

    /// 受取人の NFT トークンアカウント
    #[account(mut)]
    pub recipient_nft: Account<'info, TokenAccount>,

    /// SPL トークンプログラム
    pub token_program: Program<'info, Token>,

    /// ギフト手数料設定を保持するアカウント
    pub config: Account<'info, GiftConfig>,
}

#[account]
pub struct GiftAccount {
    /// 本来このギフト機能を操作できるべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 累計ギフト送信回数
    pub gifts_sent: u64,
    /// 最後に添えられたメモ
    pub last_note: String,
}

#[account]
pub struct GiftConfig {
    /// 1 回のギフト送信につき徴収する手数料（Lamports）
    pub gift_fee: u64,
}
