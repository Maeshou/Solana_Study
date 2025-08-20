use anchor_lang::prelude::*;
declare_id!("GiftCardVuln11111111111111111111111111111");

/// ギフトカード情報
#[account]
pub struct GiftCard {
    pub issuer:     Pubkey,       // 発行者
    pub code:       String,       // カードコード
    pub redeemers:  Vec<Pubkey>,  // 利用したユーザー一覧
}

/// ギフトカード利用記録
#[account]
pub struct RedemptionRecord {
    pub user:        Pubkey,      // 利用ユーザー
    pub gift_card:   Pubkey,      // 本来は GiftCard.key() と一致すべき
    pub message:     String,      // 利用メッセージ
}

#[derive(Accounts)]
pub struct CreateGiftCard<'info> {
    #[account(init, payer = issuer, space = 8 + 32 + 4 + 64 + 4 + (32 * 50))]
    pub gift_card:    Account<'info, GiftCard>,
    #[account(mut)]
    pub issuer:       Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RedeemGiftCard<'info> {
    /// GiftCard.issuer == issuer.key() は検証される
    #[account(mut, has_one = issuer)]
    pub gift_card:    Account<'info, GiftCard>,

    /// RedemptionRecord.gift_card ⇔ gift_card.key() の検証がないため、
    /// 偽のレコードを渡して任意のカードを利用できる
    #[account(init, payer = user, space = 8 + 32 + 32 + 4 + 128)]
    pub redemption:   Account<'info, RedemptionRecord>,

    #[account(mut)]
    pub issuer:       Signer<'info>,
    #[account(mut)]
    pub user:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClearRedeemers<'info> {
    /// RedemptionRecord.user == user.key() は検証される
    #[account(mut, has_one = user)]
    pub redemption:   Account<'info, RedemptionRecord>,

    /// gift_card.key() ⇔ redemption.gift_card の検証がないため、
    /// 偽物のレコードで別のカードの利用履歴を消去できる
    #[account(mut)]
    pub gift_card:    Account<'info, GiftCard>,

    pub user:         Signer<'info>,
}

#[program]
pub mod giftcard_vuln {
    use super::*;

    pub fn create_gift_card(ctx: Context<CreateGiftCard>, code: String) -> Result<()> {
        let gc = &mut ctx.accounts.gift_card;
        gc.issuer     = ctx.accounts.issuer.key();
        gc.code       = code;
        // redeemers は init 時に空 Vec
        Ok(())
    }

    pub fn redeem(ctx: Context<RedeemGiftCard>, message: String) -> Result<()> {
        let gc  = &mut ctx.accounts.gift_card;
        let rr  = &mut ctx.accounts.redemption;

        // 脆弱性ポイント:
        // rr.gift_card = gc.key(); の一致検証がない
        rr.user      = ctx.accounts.user.key();
        rr.gift_card = gc.key();
        rr.message   = message;

        // 利用者一覧に追加
        gc.redeemers.push(rr.user);
        Ok(())
    }

    pub fn clear_redeemers(ctx: Context<ClearRedeemers>) -> Result<()> {
        let gc = &mut ctx.accounts.gift_card;

        // 本来必要:
        // require_keys_eq!(ctx.accounts.redemption.gift_card, gc.key(), ErrorCode::Mismatch);

        // Vec::truncate で最後の利用者を除去（分岐・ループなし）
        let remaining = gc.redeemers.len().saturating_sub(1);
        gc.redeemers.truncate(remaining);
        Ok(())
    }
}
