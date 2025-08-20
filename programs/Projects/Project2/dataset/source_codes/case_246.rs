use anchor_lang::prelude::*;

declare_id!("ShopRing5050505050505050505050505050505050");

#[program]
pub mod shop_history {
    use super::*;

    pub fn init(ctx: Context<InitHistory>) -> Result<()> {
        // バッファは 0 で初期化済み
        Ok(())
    }

    pub fn record_price(ctx: Context<RecPrice>, price: u64) -> Result<()> {
        let h = &mut ctx.accounts.history;
        h.buffer[h.index as usize] = price;
        h.index = (h.index + 1) % (h.buffer.len() as u8);
        h.count = (h.count + 1).min(h.buffer.len() as u64);
        Ok(())
    }

    pub fn average(ctx: Context<ViewHistory>) -> Result<f64> {
        let h = &ctx.accounts.history;
        let mut sum = 0u128;
        for &p in h.buffer.iter().take(h.count as usize) {
            sum += p as u128;
        }
        Ok((sum as f64) / (h.count as f64))
    }
}

#[derive(Accounts)]
pub struct InitHistory<'info> {
    #[account(init, payer = admin, space = 8 + 8*8 + 1 + 8)]
    pub history: Account<'info, PriceHistory>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecPrice<'info> {
    #[account(mut)]
    pub history: Account<'info, PriceHistory>,
}

#[derive(Accounts)]
pub struct ViewHistory<'info> {
    pub history: Account<'info, PriceHistory>,
}

#[account]
pub struct PriceHistory {
    pub buffer: [u64; 8],
    pub index: u8,
    pub count: u64,
}
