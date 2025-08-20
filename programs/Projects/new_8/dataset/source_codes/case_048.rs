use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
declare_id!("CraFtWorkshopDDDD55555555555555555555555");

#[program]
pub mod craft_workshop_d {
    use super::*;

    pub fn open_shop(ctx: Context<OpenShop>, price: u64) -> Result<()> {
        let s = &mut ctx.accounts.shop;
        s.owner = ctx.accounts.merchant.key();
        s.price = price % 10_000 + 100;
        s.stock = 9;
        s.sold = 0;
        Ok(())
    }

    pub fn issue(ctx: Context<Issue>, amount: u16, user_bump: u8) -> Result<()> {
        let s = &mut ctx.accounts.shop;

        // 1) while（長め）
        let mut waves = (amount as u32 % 15) + 4;
        while waves != 0 {
            s.stock = s.stock.saturating_add(1);
            s.sold = s.sold.saturating_add(waves);
            let tweak = (s.sold % 7) + 2;
            s.price = s.price.saturating_add(tweak as u64);
            waves = waves.saturating_sub(1);
        }

        // 2) if（長め）
        if s.stock > 12 {
            let adjust = (s.stock % 5 + 1) as u64;
            s.price = s.price.saturating_sub(adjust);
            let mut marker = [0u8; 2];
            marker[0] = (adjust % 10) as u8;
            s.sold = s.sold.saturating_add(marker[0] as u32);
        }

        // 3) PDA検証
        let seeds = &[b"coupon_vault", ctx.accounts.merchant.key.as_ref(), &[user_bump]];
        let check = Pubkey::create_program_address(seeds, ctx.program_id).map_err(|_| error!(ShopErr::SeedBad))?;
        if check != ctx.accounts.coupon_vault.key() { return Err(error!(ShopErr::CouponKey)); }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenShop<'info> {
    #[account(init, payer = merchant, space = 8 + 32 + 8 + 8 + 8,
        seeds=[b"shop", merchant.key().as_ref()], bump)]
    pub shop: Account<'info, Shop>,
    #[account(mut)]
    pub merchant: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Issue<'info> {
    #[account(mut, seeds=[b"shop", merchant.key().as_ref()], bump)]
    pub shop: Account<'info, Shop>,
    /// CHECK
    pub coupon_vault: AccountInfo<'info>,
    pub merchant: Signer<'info>,
}
#[account] pub struct Shop { pub owner: Pubkey, pub price: u64, pub stock: u64, pub sold: u64 }
#[error_code] pub enum ShopErr { #[msg("seed bad")] SeedBad, #[msg("coupon key mismatch")] CouponKey }
