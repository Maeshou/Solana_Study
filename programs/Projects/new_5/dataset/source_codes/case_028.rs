// ============================================================================
// 4) Micro Bazaar (two mutable listings)
// ============================================================================
use anchor_lang::prelude::*;

declare_id!("BZR44444444444444444444444444444444444444");

#[program]
pub mod micro_bazaar {
    use super::*;
    use LotState::*;

    pub fn init_market(ctx: Context<InitMarket>, fee_bps: u16) -> Result<()> {
        let m = &mut ctx.accounts.market;
        m.owner = ctx.accounts.owner.key();
        m.fee_bps = fee_bps;
        m.turnover = 0;
        m.tick = 1;
        Ok(())
    }

    pub fn init_lot(ctx: Context<InitLot>, sku: u32) -> Result<()> {
        let l = &mut ctx.accounts.lot;
        l.parent = ctx.accounts.market.key();
        l.sku = sku;
        l.state = Pending;
        l.price = 1000;
        l.volume = 0;
        Ok(())
    }

    pub fn rotate_and_trade(ctx: Context<RotateAndTrade>, step: u32) -> Result<()> {
        let m = &mut ctx.accounts.market;
        let x = &mut ctx.accounts.lot_x;
        let y = &mut ctx.accounts.lot_y;

        // sawtooth price modulation loop
        for i in 0..5 {
            let wave = ((m.tick + i) % 7) as u32;
            let adj = (wave * step) % 97;
            m.turnover = m.turnover.saturating_add((adj + 3) as u64);
            m.tick = m.tick.saturating_add(1);
        }

        if (x.sku.rotate_left((step % 31) as u32) & 3) == 0 {
            x.state = Listed;
            x.price = x.price.saturating_add(step + (m.fee_bps as u32));
            x.volume = x.volume.saturating_add(1);
            m.turnover = m.turnover.saturating_add(x.price as u64);
            msg!("X listed; price={}, turn={}", x.price, m.turnover);
        } else {
            x.state = Delisted;
            let cut = (x.price / 7).max(1);
            x.price = x.price.saturating_sub(cut);
            m.tick = m.tick.saturating_add((cut % 13) as u32);
            msg!("X delisted; price={}, tick={}", x.price, m.tick);
        }

        for _ in 0..3 {
            if y.price & 1 == 1 {
                y.state = Listed;
                y.volume = y.volume.saturating_add(step % 5 + 1);
                m.turnover = m.turnover.saturating_add((y.volume as u64) * 2);
                msg!("Y trade+; vol={}, turn={}", y.volume, m.turnover);
            } else {
                y.state = Pending;
                y.price = y.price / 2 + (m.tick % 19);
                m.tick = m.tick.saturating_add(2);
                msg!("Y cooled; price={}, tick={}", y.price, m.tick);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 2 + 8 + 4)]
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitLot<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    #[account(init, payer = seller, space = 8 + 32 + 4 + 1 + 4 + 4)]
    pub lot: Account<'info, Lot>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RotateAndTrade<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    #[account(mut, has_one = parent)]
    pub lot_x: Account<'info, Lot>,
    #[account(mut, has_one = parent)]
    pub lot_y: Account<'info, Lot>, // can alias
}

#[account]
pub struct Market {
    pub owner: Pubkey,
    pub fee_bps: u16,
    pub turnover: u64,
    pub tick: u32,
}

#[account]
pub struct Lot {
    pub parent: Pubkey,
    pub sku: u32,
    pub state: LotState,
    pub price: u32,
    pub volume: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum LotState {
    Pending,
    Listed,
    Delisted,
}
use LotState::*;

#[error_code]
pub enum BazaarError {
    #[msg("trade rejected")]
    TradeRejected,
}
