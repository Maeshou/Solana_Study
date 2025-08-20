use anchor_lang::prelude::*;

declare_id!("SafeEx16Discount1111111111111111111111111111");

#[program]
pub mod example16 {
    use super::*;

    pub fn init_discount(
        ctx: Context<InitDiscount>,
        price: u64,
        tier: u8,
    ) -> Result<()> {
        let d = &mut ctx.accounts.discount;
        d.price = price;
        d.tier = tier;
        d.final_price = price;

        // ティアに応じて割引率を決定
        let mut rate = 0u8;
        if tier == 1 {
            rate = 5;
        } else {
            rate = 10;
        }
        // 割引計算
        let discount_amt = d.price * rate as u64 / 100;
        d.final_price = d.price.saturating_sub(discount_amt);

        // 追加ボーナス：tier 2 なら更に 1%off
        if tier == 2 {
            let bonus = d.final_price / 100;
            d.final_price = d.final_price.saturating_sub(bonus);
        }
        Ok(())
    }

    pub fn apply_discount(
        ctx: Context<ApplyDiscount>,
        extra_tier: u8,
    ) -> Result<()> {
        let d = &mut ctx.accounts.discount;
        // ティア合算
        let new_tier = d.tier.saturating_add(extra_tier);
        d.tier = if new_tier > 2 { 2 } else { new_tier };

        // 再計算
        let mut rate = 0u8;
        if d.tier == 1 {
            rate = 7;
        } else {
            rate = 12;
        }
        let disc = d.price * rate as u64 / 100;
        d.final_price = d.price.saturating_sub(disc);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDiscount<'info> {
    #[account(init, payer = user, space = 8 + 8 + 1 + 8)]
    pub discount: Account<'info, DiscountData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApplyDiscount<'info> {
    #[account(mut)] pub discount: Account<'info, DiscountData>,
}

#[account]
pub struct DiscountData {
    pub price:       u64,
    pub tier:        u8,
    pub final_price: u64,
}
