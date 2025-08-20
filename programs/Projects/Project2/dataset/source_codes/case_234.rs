use anchor_lang::prelude::*;

declare_id!("ShopDsc03333333333333333333333333333333333");

#[program]
pub mod shop_discount {
    use super::*;

    pub fn calc_price(ctx: Context<CalcPrice>, base: u64, level: u8) -> Result<Discounted> {
        let factor = 100u64.saturating_sub((level as u64).saturating_mul(2));
        let price = base.saturating_mul(factor).checked_div(100).unwrap_or(base);
        Ok(Discounted { price })
    }
}

#[derive(Accounts)]
pub struct CalcPrice<'info> {
    pub user: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Discounted {
    pub price: u64,
}
