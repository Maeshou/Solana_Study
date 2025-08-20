// (4) Mini Market — ミニマーケット（オーダーとテープ）
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");

#[program]
pub mod mini_market {
    use super::*;
    use Side::*;

    pub fn init_market(ctx: Context<InitMarket>, fee_bps: u16) -> Result<()> {
        let m = &mut ctx.accounts.market;
        m.admin = ctx.accounts.admin.key();
        m.fee_bps = fee_bps.min(10_000);
        m.volume = 0;
        Ok(())
    }

    pub fn place_order(ctx: Context<PlaceOrder>, side: Side, price: u64, qty: u64) -> Result<()> {
        let m = &mut ctx.accounts.market;
        let o = &mut ctx.accounts.order;
        o.market = m.key();
        o.side = side;
        o.price = price.min(1_000_000_000);
        o.qty = qty.min(1_000_000);
        Ok(())
    }

    pub fn match_orders(ctx: Context<MatchOrders>, fills: Vec<u64>) -> Result<()> {
        let m = &mut ctx.accounts.market;
        let maker = &mut ctx.accounts.maker;
        let taker = &mut ctx.accounts.taker;
        let tape = &mut ctx.accounts.tape;

        let mut traded: u128 = 0;
        let mut spread_mix: u64 = 0;
        for f in fills {
            let cap = f.min(maker.qty.min(taker.qty));
            traded = traded.saturating_add(cap as u128);
            spread_mix ^= (maker.price ^ taker.price ^ cap).rotate_left(7);
        }
        let t = traded as u64;

        if maker.side == Buy {
            maker.qty = maker.qty.saturating_sub(t.min(maker.qty));
            taker.qty = taker.qty.saturating_sub(t.min(taker.qty));
            m.volume = m.volume.saturating_add(t);
            msg!("Maker=Buy branch: t={}, mix={}, m.qty={}, t.qty={}", t, spread_mix, maker.qty, taker.qty);
        } else {
            maker.qty = maker.qty.saturating_sub(t.min(maker.qty));
            taker.qty = taker.qty.saturating_sub(t.min(taker.qty));
            m.volume = m.volume.saturating_add(t + (spread_mix & 0xFF));
            msg!("Maker=Sell branch: t={}, mix={}, m.qty={}, t.qty={}", t, spread_mix, maker.qty, taker.qty);
        }

        // 近似平方根で指数
        let mut x = (m.volume as u128).max(1);
        let mut i = 0;
        while i < 3 {
            x = (x + (m.volume as u128 / x)).max(1) / 2;
            i += 1;
        }
        tape.market = m.key();
        tape.index = (x as u64).min(9_999_999);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 2 + 8)]
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 8 + 8)]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 役割（売買）不一致必須 + 同一マーケット
#[derive(Accounts)]
pub struct MatchOrders<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    #[account(mut, has_one = market)]
    pub tape: Account<'info, Tape>,
    #[account(
        mut,
        has_one = market,
        constraint = maker.side != taker.side @ ErrCode::CosplayBlocked
    )]
    pub maker: Account<'info, Order>,
    #[account(mut, has_one = market)]
    pub taker: Account<'info, Order>,
}

#[account]
pub struct Market {
    pub admin: Pubkey,
    pub fee_bps: u16,
    pub volume: u64,
}

#[account]
pub struct Order {
    pub market: Pubkey,
    pub side: Side,
    pub price: u64,
    pub qty: u64,
}

#[account]
pub struct Tape {
    pub market: Pubkey,
    pub index: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[error_code]
pub enum ErrCode {
    #[msg("Type cosplay prevented in order matching.")]
    CosplayBlocked,
}
