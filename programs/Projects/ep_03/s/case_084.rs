use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgTradeSvc01");

#[program]
pub mod trade_service {
    use super::*;

    /// NFTトレードを出品するが、
    /// trade_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn offer_trade(ctx: Context<ModifyTrade>, price: u64) -> Result<()> {
        let acct = &mut ctx.accounts.trade_account;
        record_offer(acct, price);
        Ok(())
    }

    /// 出品を取り下げるが、
    /// trade_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn cancel_trade(ctx: Context<ModifyTrade>) -> Result<()> {
        let acct = &mut ctx.accounts.trade_account;
        record_cancel(acct);
        Ok(())
    }
}

/// 出品情報を更新してカウンタをインクリメントするヘルパー
fn record_offer(acct: &mut TradeAccount, price: u64) {
    acct.price = price;
    acct.active = true;
    acct.offer_count = acct.offer_count.checked_add(1).unwrap();
}

/// 出品を取り下げてカウンタをインクリメントするヘルパー
fn record_cancel(acct: &mut TradeAccount) {
    acct.active = false;
    acct.cancel_count = acct.cancel_count.checked_add(1).unwrap();
}

#[derive(Accounts)]
pub struct ModifyTrade<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub trade_account: Account<'info, TradeAccount>,

    /// 操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct TradeAccount {
    /// 本来この出品を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 出品中のNFTミントアドレス
    pub nft_mint: Pubkey,
    /// 設定された価格（Lamports）
    pub price: u64,
    /// 出品中フラグ
    pub active: bool,
    /// 出品操作の累計回数
    pub offer_count: u64,
    /// 出品取り下げ操作の累計回数
    pub cancel_count: u64,
}
