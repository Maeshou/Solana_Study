// ============================================================================
// 6) Bazaar Curves — 幾何平均/整数sqrt（価格曲線）— PDAなし
// ============================================================================
declare_id!("BZCV666666666666666666666666666666666");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ListingState { Live, Cooling, Closed }

fn isqrt(n: u128) -> u128 {
    // 整数平方根（バビロニア法）
    if n == 0 { return 0; }
    let mut x = n;
    let mut y = (x + 1) >> 1;
    while y < x { x = y; y = (x + n / x) >> 1; }
    x
}

#[program]
pub mod bazaar_curves {
    use super::*;

    pub fn init_listing(ctx: Context<InitListing>, base: u64) -> Result<()> {
        let m = &mut ctx.accounts;
        m.listing.seller = m.seller.key();
        m.listing.base = base;
        m.listing.state = ListingState::Live;
        Ok(())
    }

    pub fn buy(ctx: Context<Buy>, qty: u32) -> Result<()> {
        let m = &mut ctx.accounts;
        assert_ne!(m.listing.key(), m.wallet.key(), "listing/wallet must differ");

        for _ in 0..qty {
            // 幾何平均っぽい単調価格： new = sqrt(old * (old + k))
            let k = 17u128;
            let old = m.stats.price as u128 + 1;
            let newp = isqrt(old.saturating_mul(old + k));
            m.stats.price = (newp.min(u128::from(u64::MAX))) as u64;

            m.wallet.lock = m.wallet.lock.wrapping_add(1);
            m.stats.trades = m.stats.trades.wrapping_add(1);
        }

        if m.stats.price > m.listing.base * 20 {
            m.listing.state = ListingState::Closed;
            m.wallet.balance = m.wallet.balance.saturating_sub(1).max(0);
            m.stats.disputes = m.stats.disputes.wrapping_add(1);
            msg!("closed: price high; balance-1, disputes+1");
        } else {
            m.listing.state = ListingState::Cooling;
            m.wallet.balance = m.wallet.balance.saturating_add(2);
            m.stats.trades = m.stats.trades.wrapping_mul(2);
            msg!("cooling: balance+2, trades*=2");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitListing<'info> {
    #[account(init, payer=payer, space=8+32+8+1)]
    pub listing: Account<'info, Listing>,
    #[account(init, payer=payer, space=8+32+8+1)]
    pub wallet: Account<'info, Wallet>,
    #[account(init, payer=payer, space=8+8+4)]
    pub stats: Account<'info, MarketStats>,
    #[account(mut)] pub payer: Signer<'info>,
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut, has_one=seller, constraint = listing.key() != stats.key(), error = CurveErr::Dup)]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub stats: Account<'info, MarketStats>,
    pub seller: Signer<'info>,
}

#[account] pub struct Listing { pub seller: Pubkey, pub base: u64, pub state: ListingState }
#[account] pub struct Wallet { pub owner: Pubkey, pub balance: u64, pub lock: u8 }
#[account] pub struct MarketStats { pub price: u64, pub trades: u32, pub disputes: u32 }

#[error_code] pub enum CurveErr { #[msg("dup")] Dup }

