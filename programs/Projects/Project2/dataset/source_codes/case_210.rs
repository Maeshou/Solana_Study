use anchor_lang::prelude::*;

declare_id!("Price7777777777777777777777777777777777");

#[program]
pub mod price_manager {
    use super::*;

    pub fn init_price(ctx: Context<InitPrice>, floor: u64, ceiling: u64) -> Result<()> {
        let p = &mut ctx.accounts.price;
        p.floor = floor;
        p.ceiling = ceiling;
        p.current = (floor + ceiling) / 2;
        Ok(())
    }

    pub fn adjust(ctx: Context<ModifyPrice>, delta: i64) -> Result<()> {
        let p = &mut ctx.accounts.price;
        let new = (p.current as i64).saturating_add(delta);
        p.current = new.clamp(p.floor as i64, p.ceiling as i64) as u64;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPrice<'info> {
    #[account(init, payer = admin, space = 8 + 8*3)]
    pub price: Account<'info, PriceData>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyPrice<'info> {
    #[account(mut)] pub price: Account<'info, PriceData>,
}

#[account]
pub struct PriceData {
    pub floor: u64,
    pub ceiling: u64,
    pub current: u64,
}
