// ======================================================================
// 3) Trinket Bazaar：骨董バザール（PDAなし / has_one + 不一致）
//    - init_market：Xorshiftで在庫初期化
//    - trade_once：ループ・分岐なしで在庫・売上・ローリング値を更新
// ======================================================================
declare_id!("BZAR3434343434343434343434343434343434343434");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BazaarState { Setup, Trade, Close }

#[program]
pub mod trinket_bazaar {
    use super::*;
    use BazaarState::*;

    fn xorshift(mut z: u32) -> u32 {
        z ^= z << 13;
        z ^= z >> 17;
        z ^= z << 5;
        z
    }

    pub fn init_market(ctx: Context<InitMarket>, seed: u32) -> Result<()> {
        let m = &mut ctx.accounts.market;
        m.owner = ctx.accounts.merchant.key();
        m.limit = (seed as u64) * 7 + 1_000;
        m.state = Setup;

        let a = &mut ctx.accounts.stall_a;
        let b = &mut ctx.accounts.stall_b;

        let s1 = xorshift(seed);
        let s2 = xorshift(seed ^ 0x9E37_79B9);

        a.parent = m.key();
        a.aisle = (s1 as u8) & 7;
        a.stock = (s1 & 0x3FF) + 60;

        b.parent = m.key();
        b.aisle = ((s2 >> 3) as u8) & 7;
        b.stock = ((s2 >> 5) & 0x3FF) + 55;

        let c = &mut ctx.accounts.clerk;
        c.parent = m.key();
        c.desk = 9;
        c.receipts = 0;
        c.rolling = s1 ^ s2;

        Ok(())
    }

    pub fn trade_once(ctx: Context<TradeOnce>) -> Result<()> {
        let m = &mut ctx.accounts.market;
        let a = &mut ctx.accounts.stall_a;
        let b = &mut ctx.accounts.stall_b;
        let c = &mut ctx.accounts.clerk;

        let h = ((a.stock ^ b.stock) as u64).wrapping_mul(1469598103934665603);
        let take = ((h & 15) + 5) as u32;
        a.stock = a.stock.saturating_sub(take.min(a.stock));
        b.stock = b.stock.checked_add(take + 3).unwrap_or(u32::MAX);
        c.receipts = c.receipts.saturating_add(take as u64 + ((b.stock & 7) as u64));
        c.rolling ^= (h as u32).rotate_left(7);
        m.state = Trade;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(init, payer=payer, space=8 + 32 + 8 + 1)]
    pub market: Account<'info, Market>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub stall_a: Account<'info, Stall>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub stall_b: Account<'info, Stall>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 4)]
    pub clerk: Account<'info, ClerkTape>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub merchant: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TradeOnce<'info> {
    #[account(mut, has_one=owner)]
    pub market: Account<'info, Market>,
    #[account(
        mut,
        has_one=market,
        constraint = stall_a.aisle != stall_b.aisle @ BazaarErr::Dup
    )]
    pub stall_a: Account<'info, Stall>,
    #[account(
        mut,
        has_one=market,
        constraint = stall_b.aisle != clerk.desk @ BazaarErr::Dup
    )]
    pub stall_b: Account<'info, Stall>,
    #[account(mut, has_one=market)]
    pub clerk: Account<'info, ClerkTape>,
    pub merchant: Signer<'info>,
}

#[account]
pub struct Market {
    pub owner: Pubkey,
    pub limit: u64,
    pub state: BazaarState,
}

#[account]
pub struct Stall {
    pub parent: Pubkey,
    pub aisle: u8,   // 一意フィールド
    pub stock: u32,
}

#[account]
pub struct ClerkTape {
    pub parent: Pubkey,
    pub desk: u8,    // 一意フィールド
    pub receipts: u64,
    pub rolling: u32,
}

#[error_code]
pub enum BazaarErr {
    #[msg("duplicate mutable account")]
    Dup,
}


