// ===============================================
// (6) mini_market: ミニマーケット（注文・板・倉庫）
//   - 多層防御: has_one + side 不一致 + Token を未使用（純粋ロジック）
// ===============================================
use anchor_lang::prelude::*;
declare_id!("Min1MarKet666666666666666666666666666666");

#[program]
pub mod mini_market {
    use super::*;

    pub fn init_book(ctx: Context<InitBook>) -> Result<()> {
        let b = &mut ctx.accounts.order_book;
        b.owner = ctx.accounts.owner.key();
        b.mid = 0;
        b.liq = 0;
        b.hist = [0u32; 8];
        Ok(())
    }

    pub fn open_order(ctx: Context<OpenOrder>, side: Side, price: u32, qty: u32) -> Result<()> {
        let o = &mut ctx.accounts.order;
        o.parent = ctx.accounts.order_book.key();
        o.trader = ctx.accounts.trader.key();
        o.side = side;
        o.price = price;
        o.qty = qty;
        Ok(())
    }

    pub fn init_warehouse(ctx: Context<InitWarehouse>) -> Result<()> {
        let w = &mut ctx.accounts.warehouse;
        w.parent = ctx.accounts.order_book.key();
        w.filled = 0;
        w.hash = 0;
        w.last = 0;
        Ok(())
    }

    pub fn match_orders(ctx: Context<MatchOrders>) -> Result<()> {
        let book = &mut ctx.accounts.order_book;
        let a = &mut ctx.accounts.order_a;
        let b = &mut ctx.accounts.order_b;
        let w = &mut ctx.accounts.warehouse;

        // 価格帯で簡易マッチング: bid<=ask なら約定
        let cross = if a.side == Side::Bid {
            a.price as i64 - b.price as i64
        } else {
            b.price as i64 - a.price as i64
        };
        let can_match = cross >= 0;

        // 板の中央値擬似更新（履歴ベース）
        let mut sum: u64 = 0;
        for i in 0..book.hist.len() {
            let slot = (book.hist[i] as u64).wrapping_add((a.price as u64 ^ b.price as u64) & 0xFFFF);
            book.hist[i] = (slot & 0xFFFF) as u32;
            sum = sum.wrapping_add(book.hist[i] as u64);
        }
        let avg = (sum / (book.hist.len() as u64)).min(u32::MAX as u64) as u32;
        book.mid = avg;

        if can_match {
            // 約定数量（最小数量）
            let fill = a.qty.min(b.qty);
            a.qty = a.qty.saturating_sub(fill);
            b.qty = b.qty.saturating_sub(fill);
            w.filled = w.filled.saturating_add(fill as u64);
            w.last = fill;
            // 簡易なハッシュ更新
            w.hash ^= (fill as u64).rotate_left(((a.price ^ b.price) & 31) as u32);
            // 板の流動性加算
            book.liq = book.liq.saturating_add(fill as u64);
            msg!("matched: fill={}, book_mid={}, liq={}", fill, book.mid, book.liq);
        } else {
            // 約定不可: 価格差に応じてヒストリだけ進める
            let drift = cross.unsigned_abs().min(10_000) as u32;
            book.hist[0] = book.hist[0].saturating_add(drift);
            w.last = 0;
            msg!("no match: drift={}, book_mid={}", drift, book.mid);
        }
        Ok(())
    }
}

// -------------------- Accounts --------------------

#[derive(Accounts)]
pub struct InitBook<'info> {
    #[account(
        init,
        payer = owner,
        // 8 + 32(owner) + 4(mid) + 8(liq) + 4*8(hist)
        space = 8 + 32 + 4 + 8 + (4*8)
    )]
    pub order_book: Account<'info, OrderBook>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OpenOrder<'info> {
    #[account(mut)]
    pub order_book: Account<'info, OrderBook>,
    #[account(
        init,
        payer = trader,
        // 8 + 32(parent) + 32(trader) + 1(side) + 4(price) + 4(qty)
        space = 8 + 32 + 32 + 1 + 4 + 4
    )]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub trader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitWarehouse<'info> {
    #[account(mut)]
    pub order_book: Account<'info, OrderBook>,
    #[account(
        init,
        payer = owner,
        // 8 + 32(parent) + 8(filled) + 8(hash) + 4(last)
        space = 8 + 32 + 8 + 8 + 4
    )]
    pub warehouse: Account<'info, Warehouse>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MatchOrders<'info> {
    #[account(mut)]
    pub order_book: Account<'info, OrderBook>,
    #[account(
        mut,
        constraint = order_a.parent == order_book.key() @ MarketErr::Cosplay
    )]
    pub order_a: Account<'info, Order>,
    #[account(
        mut,
        constraint = order_b.parent == order_book.key() @ MarketErr::Cosplay,
        // Side 不一致（同一口座や同一 side の取り回しを防止）
        constraint = order_a.side as u8 != order_b.side as u8 @ MarketErr::Cosplay
    )]
    pub order_b: Account<'info, Order>,
    #[account(
        mut,
        constraint = warehouse.parent == order_book.key() @ MarketErr::Cosplay
    )]
    pub warehouse: Account<'info, Warehouse>,
}

// -------------------- Data --------------------

#[account]
pub struct OrderBook {
    pub owner: Pubkey,
    pub mid: u32,
    pub liq: u64,
    pub hist: [u32; 8],
}

#[account]
pub struct Order {
    pub parent: Pubkey, // = order_book
    pub trader: Pubkey,
    pub side: Side,
    pub price: u32,
    pub qty: u32,
}

#[account]
pub struct Warehouse {
    pub parent: Pubkey, // = order_book
    pub filled: u64,
    pub hash: u64,
    pub last: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Side { Bid, Ask }

#[error_code]
pub enum MarketErr { #[msg("cosplay blocked")] Cosplay }
