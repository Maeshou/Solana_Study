// 9) Market Curves Dist — ロジスティック近似（Q16.16）PDAなし
declare_id!("MKDS999999999999999999999999999999999");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum MarketState { Live, Cool, Shut }

#[program]
pub mod market_curves_dist {
    use super::*;
    use MarketState::*;

    pub fn init_market(ctx: Context<InitMarket>, k_q16: u32) -> Result<()> {
        let m = &mut ctx.accounts;
        m.listing.seller = m.seller.key();
        m.listing.k_q16 = k_q16.max(1);
        m.listing.state = Live;
        Ok(())
    }

    pub fn quote(ctx: Context<Quote>, steps: u32) -> Result<()> {
        let m = &mut ctx.accounts;

        for i in 0..steps {
            // logistic: x_{t+1} = x + k*x*(1-x)
            let x = m.curve.x_q16 as u128;
            let k = m.listing.k_q16 as u128;
            let one = 1u128 << 16;
            let term = (x * (one - x)) >> 16;
            let nx = x + ((k * term) >> 16);
            m.curve.x_q16 = nx.min(u128::from(u32::MAX)) as u32;
            m.stats.ticks = m.stats.ticks.wrapping_add((i & 3) as u32 + 1);
        }

        if m.curve.x_q16 > (one_u32() * 3 / 4) {
            m.listing.state = Shut;
            m.stats.flags = m.stats.flags.wrapping_add(1);
            m.curve.x_q16 = (m.curve.x_q16 / 2).max(1<<8);
            m.wallet.balance = m.wallet.balance.saturating_add(5);
            msg!("shut: x halved, flags+1, balance+5");
        } else {
            m.listing.state = Cool;
            m.stats.trades = m.stats.trades.wrapping_add(2);
            m.curve.x_q16 = m.curve.x_q16 + 77;
            m.wallet.lock = m.wallet.lock.wrapping_add(1);
            msg!("cool: trades+2, x+77, lock+1");
        }
        Ok(())
    }
    fn one_u32() -> u32 { 1 << 16 }
}

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub listing: Account<'info, ListingCfg>,
    #[account(init, payer=payer, space=8+4)]
    pub curve: Account<'info, CurveQ16>,
    #[account(init, payer=payer, space=8+32+8+1)]
    pub wallet: Account<'info, Wallet>,
    #[account(init, payer=payer, space=8+4+8)]
    pub stats: Account<'info, MktStats>,
    #[account(mut)] pub payer: Signer<'info>,
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Quote<'info> {
    #[account(mut, has_one=seller)]
    pub listing: Account<'info, ListingCfg>,
    #[account(
        mut,
        constraint = curve.key() != listing.key() @ MkdsErr::Dup,
        constraint = curve.key() != wallet.key() @ MkdsErr::Dup
    )]
    pub curve: Account<'info, CurveQ16>,
    #[account(
        mut,
        constraint = wallet.key() != listing.key() @ MkdsErr::Dup
    )]
    pub wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub stats: Account<'info, MktStats>,
    pub seller: Signer<'info>,
}
#[account] pub struct ListingCfg { pub seller: Pubkey, pub k_q16: u32, pub state: MarketState }
#[account] pub struct CurveQ16 { pub x_q16: u32 }
#[account] pub struct Wallet { pub owner: Pubkey, pub balance: u64, pub lock: u8 }
#[account] pub struct MktStats { pub ticks: u32, pub trades: u32, pub flags: u32 }
#[error_code] pub enum MkdsErr { #[msg("dup")] Dup }
