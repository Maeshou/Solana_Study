use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("Pa1etteShop44444444444444444444444444444");

#[program]
pub mod palette_shop {
    use super::*;

    pub fn init_shop(ctx: Context<InitShop>, price: u64) -> Result<()> {
        let s = &mut ctx.accounts.shop;
        s.owner = ctx.accounts.owner.key();
        s.price = price;
        s.stock = 6;
        s.sold = 0;
        if s.price < 4 { s.price = 4; }
        Ok(())
    }

    pub fn buy(ctx: Context<Buy>, amount: u16, user_bump: u8) -> Result<()> {
        let s = &mut ctx.accounts.shop;

        // 手動導出: coupon_box
        let seeds = &[
            b"coupon_box",
            ctx.accounts.owner.key.as_ref(),
            &[user_bump],
        ];
        let c = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(ShopErr::Seed))?;

        if c != ctx.accounts.coupon_box.key() {
            return Err(error!(ShopErr::CouponRoute));
        }

        let mut cnt = amount as u64;
        if cnt > 9 { cnt = 9; }
        if s.stock > 0 {
            s.stock = s.stock.saturating_sub(1);
            s.sold = s.sold.saturating_add(1);
        }

        let mut k = 1u64;
        while k < cnt {
            s.sold = s.sold.saturating_add(1);
            k = k.saturating_add(2);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitShop<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8,
        seeds=[b"shop", owner.key().as_ref()], bump)]
    pub shop: Account<'info, Shop>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut,
        seeds=[b"shop", owner.key().as_ref()], bump)]
    pub shop: Account<'info, Shop>,
    /// CHECK: 手動 bump で検証される coupon_box
    pub coupon_box: AccountInfo<'info>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Shop {
    pub owner: Pubkey,
    pub price: u64,
    pub stock: u64,
    pub sold: u64,
}

#[error_code]
pub enum ShopErr {
    #[msg("seed error")]
    Seed,
    #[msg("coupon route mismatch")]
    CouponRoute,
}
