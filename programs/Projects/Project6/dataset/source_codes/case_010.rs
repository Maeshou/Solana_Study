// ===============================================
// (10) guild_shop: ギルド内トレード（カタログ / 注文 / 精算）
//   - 多層防御: has_one + card.kind 不一致
// ===============================================
use anchor_lang::prelude::*;
declare_id!("Gu1LdSh0pAAAAABBBBBCCCCCDDDDDEEEEEFFFFF");

#[program]
pub mod guild_shop {
    use super::*;
    use Kind::*;

    pub fn init_catalog(ctx: Context<InitCatalog>) -> Result<()> {
        let c = &mut ctx.accounts.catalog;
        c.owner = ctx.accounts.owner.key();
        c.index = 0;
        c.hash = 0;
        Ok(())
    }

    pub fn issue_card(ctx: Context<IssueCard>, kind: Kind, init_balance: u64) -> Result<()> {
        let card = &mut ctx.accounts.card;
        card.parent = ctx.accounts.catalog.key();
        card.kind = kind;
        card.balance = init_balance;
        Ok(())
    }

    pub fn place_order(ctx: Context<PlaceOrder>, price: u32, qty: u32) -> Result<()> {
        let o = &mut ctx.accounts.order;
        o.parent = ctx.accounts.catalog.key();
        o.price = price;
        o.qty = qty;
        Ok(())
    }

    pub fn settle_trade(ctx: Context<SettleTrade>) -> Result<()> {
        let cat = &mut ctx.accounts.catalog;
        let buyer = &mut ctx.accounts.card_buyer;
        let seller = &mut ctx.accounts.card_seller;
        let order = &mut ctx.accounts.order;
        let rec = &mut ctx.accounts.record;

        // 売買可能数量
        let cost = (order.price as u64).saturating_mul(order.qty as u64);
        if buyer.balance >= cost {
            buyer.balance = buyer.balance.saturating_sub(cost);
            seller.balance = seller.balance.saturating_add(cost);
            rec.ok = rec.ok.saturating_add(1);
        } else {
            rec.ng = rec.ng.saturating_add(1);
        }

        // カタログ状態更新
        cat.index = cat.index.wrapping_add(order.qty as u64);
        cat.hash ^= ((order.price as u64) << 7) ^ (order.qty as u64);
        Ok(())
    }
}

// -------------------- Accounts --------------------

#[derive(Accounts)]
pub struct InitCatalog<'info> {
    #[account(
        init,
        payer = owner,
        // 8 + 32(owner) + 8(index) + 8(hash)
        space = 8 + 32 + 8 + 8
    )]
    pub catalog: Account<'info, Catalog>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IssueCard<'info> {
    #[account(mut)]
    pub catalog: Account<'info, Catalog>,
    #[account(
        init,
        payer = owner,
        // 8 + 32(parent) + 1(kind) + 8(balance)
        space = 8 + 32 + 1 + 8
    )]
    pub card: Account<'info, Card>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    #[account(mut)]
    pub catalog: Account<'info, Catalog>,
    #[account(
        init,
        payer = owner,
        // 8 + 32(parent) + 4(price) + 4(qty)
        space = 8 + 32 + 4 + 4
    )]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SettleTrade<'info> {
    #[account(mut)]
    pub catalog: Account<'info, Catalog>,
    #[account(
        mut,
        constraint = card_buyer.parent == catalog.key() @ ShopErr::Cosplay
    )]
    pub card_buyer: Account<'info, Card>,
    #[account(
        mut,
        constraint = card_seller.parent == catalog.key() @ ShopErr::Cosplay,
        constraint = card_buyer.key() != card_seller.key() @ ShopErr::Cosplay,
        constraint = card_buyer.kind as u8 != card_seller.kind as u8 @ ShopErr::Cosplay
    )]
    pub card_seller: Account<'info, Card>,
    #[account(
        mut,
        constraint = order.parent == catalog.key() @ ShopErr::Cosplay
    )]
    pub order: Account<'info, Order>,
    #[account(
        init_if_needed,
        payer = owner,
        // 8 + 32(parent) + 8(ok) + 8(ng)
        space = 8 + 32 + 8 + 8
    )]
    pub record: Account<'info, TradeRecord>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// -------------------- Data --------------------

#[account]
pub struct Catalog {
    pub owner: Pubkey,
    pub index: u64,
    pub hash: u64,
}

#[account]
pub struct Order {
    pub parent: Pubkey, // = catalog
    pub price: u32,
    pub qty: u32,
}

#[account]
pub struct Card {
    pub parent: Pubkey, // = catalog
    pub kind: Kind,
    pub balance: u64,
}

#[account]
pub struct TradeRecord {
    pub parent: Pubkey,
    pub ok: u64,
    pub ng: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Kind { Buyer, Seller, Broker }

#[error_code]
pub enum ShopErr { #[msg("cosplay blocked")] Cosplay }
