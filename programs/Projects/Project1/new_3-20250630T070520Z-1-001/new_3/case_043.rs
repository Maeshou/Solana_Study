use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSaleSvc001");

#[program]
pub mod sale_service {
    use super::*;

    /// アイテムをショップに売却し、通貨を受け取るが、
    /// item_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn sell_item(ctx: Context<SellItem>) -> Result<()> {
        let item_acc = &mut ctx.accounts.item_account;
        let cfg = &ctx.accounts.config;
        let price = cfg.sale_price;

        // 1. 売却済みフラグを立てる
        item_acc.sold = true;

        // 2. 売却価格を記録
        item_acc.sold_price = price;

        // 3. ショップの財務口座からユーザーへ通貨を移動
        **ctx.accounts.shop_treasury.to_account_info().lamports.borrow_mut() -= price;
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += price;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SellItem<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub item_account: Account<'info, InventoryItem>,

    /// 売却を実行するユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 売却金を保管するショップ財務口座
    #[account(mut)]
    pub shop_treasury: AccountInfo<'info>,

    /// 売却価格設定を保持するアカウント
    pub config: Account<'info, SaleConfig>,
}

#[account]
pub struct InventoryItem {
    /// このアイテムを所有すべきユーザーの Pubkey
    pub owner: Pubkey,
    /// NFTのミントアドレス
    pub mint: Pubkey,
    /// 売却済みかどうか
    pub sold: bool,
    /// 実際に売却された価格
    pub sold_price: u64,
}

#[account]
pub struct SaleConfig {
    /// 1アイテムあたりの売却価格（Lamports）
    pub sale_price: u64,
}
