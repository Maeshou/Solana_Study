// ======================================================================
// 4) Mini-market listings
// ======================================================================
declare_id!("MARK444444444444444444444444444444444444444");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum MarketState { Dormant, Open, Halt }

#[program]
pub mod micro_market {
    use super::*;
    use MarketState::*;

    pub fn init_market(ctx: Context<InitMarket>, fee: u16) -> Result<()> {
        let m = &mut ctx.accounts.market;
        let l1 = &mut ctx.accounts.list_a;
        let l2 = &mut ctx.accounts.list_b;
        let e = &mut ctx.accounts.escrow;

        m.owner = ctx.accounts.operator.key();
        m.fee_bps = fee;
        m.state = Dormant;

        l1.parent = m.key();
        l1.channel = 3;
        l1.price = 120;

        l2.parent = m.key();
        l2.channel = 5;
        l2.price = 140;

        e.parent = m.key();
        e.channel = 9;
        e.locked = 0;
        e.volume = 0;

        Ok(())
    }

    pub fn rotate_books(ctx: Context<RotateBooks>, turns: u32) -> Result<()> {
        let m = &mut ctx.accounts.market;
        let l1 = &mut ctx.accounts.list_a;
        let l2 = &mut ctx.accounts.list_b;
        let e = &mut ctx.accounts.escrow;

        for t in 0..turns {
            let spread = (l2.price as i64 - l1.price as i64).unsigned_abs();
            e.locked = e.locked.checked_add(spread).unwrap_or(u64::MAX);
            l1.price = l1.price.saturating_add((t % 7) as u32 + 2);
            l2.price = l2.price.saturating_sub(((t * 3) % 11) as u32);
            e.volume ^= ((l1.price ^ l2.price) as u64) << (t % 8);
        }

        if l1.price > l2.price {
            m.state = Halt;
            e.locked = e.locked.saturating_add(100);
            l1.price = (l1.price / 2) + 33;
            l2.price = (l2.price / 2) + 21;
            msg!("halt: compress prices, add locked");
        } else {
            m.state = Open;
            e.volume = e.volume.saturating_add(55);
            l1.price = l1.price.checked_add(13).unwrap_or(u32::MAX);
            l2.price ^= 0x0F0F_F0F0;
            msg!("open: price tweak & xor");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(init, payer=payer, space=8 + 32 + 2 + 1)]
    pub market: Account<'info, MarketCore>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub list_a: Account<'info, Listing>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub list_b: Account<'info, Listing>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub escrow: Account<'info, EscrowBox>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RotateBooks<'info> {
    #[account(mut, has_one=owner)]
    pub market: Account<'info, MarketCore>,
    #[account(
        mut,
        has_one=market,
        constraint = list_a.channel != list_b.channel @ MarketErr::Dup
    )]
    pub list_a: Account<'info, Listing>,
    #[account(
        mut,
        has_one=market,
        constraint = list_b.channel != escrow.channel @ MarketErr::Dup
    )]
    pub list_b: Account<'info, Listing>,
    #[account(mut, has_one=market)]
    pub escrow: Account<'info, EscrowBox>,
    pub operator: Signer<'info>,
}

#[account]
pub struct MarketCore {
    pub owner: Pubkey,
    pub fee_bps: u16,
    pub state: MarketState,
}

#[account]
pub struct Listing {
    pub parent: Pubkey,
    pub channel: u8,
    pub price: u32,
}

#[account]
pub struct EscrowBox {
    pub parent: Pubkey,
    pub channel: u8,
    pub locked: u64,
    pub volume: u64,
}

#[error_code]
pub enum MarketErr {
    #[msg("duplicate mutable account")]
    Dup,
}
