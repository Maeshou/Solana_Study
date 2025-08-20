use anchor_lang::prelude::*;
declare_id!("GiftCard1111111111111111111111111111111111");

/// ギフトカードの定義
#[account]
pub struct GiftCard {
    pub issuer:  Pubkey, // 発行者
    pub balance: u64,    // カード全体の残高
}

/// 個別カード保有情報
#[account]
pub struct GiftCardAccount {
    pub holder:     Pubkey, // 保持者
    pub gift_card:  Pubkey, // 本来は GiftCard.key() と一致すべき
    pub used_amount: u64,   // これまでに使用した金額
}

#[derive(Accounts)]
pub struct IssueCard<'info> {
    /// GiftCard.issuer == issuer.key() の検証あり
    #[account(init, payer = issuer, space = 8 + 32 + 8)]
    pub gift_card:    Account<'info, GiftCard>,
    #[account(mut)]
    pub issuer:       Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LoadFunds<'info> {
    /// GiftCard.issuer == issuer.key() の検証あり
    #[account(mut, has_one = issuer)]
    pub gift_card:    Account<'info, GiftCard>,

    /// GiftCardAccount.gift_card == gift_card.key() の検証がない
    #[account(init, payer = user, space = 8 + 32 + 32 + 8)]
    pub card_account: Account<'info, GiftCardAccount>,

    #[account(mut)]
    pub user:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RedeemGift<'info> {
    /// GiftCard.issuer == issuer.key() の検証あり
    #[account(mut, has_one = issuer)]
    pub gift_card:    Account<'info, GiftCard>,

    /// GiftCardAccount.gift_card と gift_card.key() の一致検証がない
    #[account(mut)]
    pub card_account: Account<'info, GiftCardAccount>,

    pub issuer:       Signer<'info>,
}

#[program]
pub mod giftcard_vuln {
    use super::*;

    /// ギフトカードを発行
    pub fn issue_card(ctx: Context<IssueCard>, initial_balance: u64) -> Result<()> {
        let gc = &mut ctx.accounts.gift_card;
        gc.issuer  = ctx.accounts.issuer.key();
        gc.balance = initial_balance;
        Ok(())
    }

    /// ギフトカードに残高をチャージし、カードアカウントを初期化
    pub fn load_funds(ctx: Context<LoadFunds>, amount: u64) -> Result<()> {
        let gc = &mut ctx.accounts.gift_card;
        let ca = &mut ctx.accounts.card_account;
        // 脆弱性ポイント：
        // ca.gift_card = gc.key(); と設定するだけで、
        // GiftCardAccount.gift_card と GiftCard.key() の検証は一切入っていない
        ca.holder      = ctx.accounts.user.key();
        ca.gift_card   = gc.key();
        ca.used_amount = 0;
        gc.balance     = gc.balance.checked_add(amount).unwrap();
        Ok(())
    }

    /// ギフトカードを利用（残高から差引）
    pub fn redeem_gift(ctx: Context<RedeemGift>, spend: u64) -> Result<()> {
        let gc = &mut ctx.accounts.gift_card;
        let ca = &mut ctx.accounts.card_account;
        // 本来は必須：
        // require_keys_eq!(
        //     ca.gift_card,
        //     gc.key(),
        //     GiftError::CardMismatch
        // );
        // がないため、攻撃者は自分で用意した任意の GiftCardAccount を渡して
        // 任意のカード残高を減らせてしまう
        ca.used_amount = ca.used_amount.checked_add(spend).unwrap();
        gc.balance     = gc.balance.checked_sub(spend).unwrap();
        Ok(())
    }
}

#[error_code]
pub enum GiftError {
    #[msg("GiftCardAccount が指定の GiftCard と一致しません")]
    CardMismatch,
}
