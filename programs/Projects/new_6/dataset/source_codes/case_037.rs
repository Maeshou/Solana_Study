use anchor_lang::prelude::*;

declare_id!("NftM4rk3tTrd12222222222222222222222222222222");

#[program]
pub mod nft_market {
    use super::*;

    pub fn init_order(ctx: Context<InitOrder>, price: u64) -> Result<()> {
        let o = &mut ctx.accounts.order;
        o.seller = ctx.accounts.user.key();
        o.price = price;
        o.filled = false;
        o.retry_count = 0;
        o.priority = 0;
        Ok(())
    }

    pub fn act_fill(ctx: Context<FillOrder>, paid: u64, boost: bool) -> Result<()> {
        let o = &mut ctx.accounts.order;
        let payer = &ctx.accounts.payer;

        if paid >= o.price {
            o.filled = true;
            o.retry_count = 0;
            o.priority = o.priority.saturating_add(2);
            o.seller = payer.key(); // Type Cosplay：決済者で上書き可能
        }

        if boost {
            o.priority = o.priority.saturating_add(10);
            o.price = o.price.saturating_mul(2);
            o.retry_count += 1;
        }

        if o.retry_count > 3 {
            o.priority = 0;
            o.price = 0;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitOrder<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 1 + 1 + 1)]
    pub order: Account<'info, MarketOrder>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FillOrder<'info> {
    #[account(mut)]
    pub order: Account<'info, MarketOrder>,
    /// CHECK: buyer としても seller としても通る構造
    pub payer: AccountInfo<'info>,
}

#[account]
pub struct MarketOrder {
    pub seller: Pubkey,
    pub price: u64,
    pub filled: bool,
    pub retry_count: u8,
    pub priority: u8,
}
