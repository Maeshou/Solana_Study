use anchor_lang::prelude::*;

declare_id!("Var10MktAAAAAAAABBBBBBBBBCCCCCCCCDDDDDDDD");

#[program]
pub mod varied_market {
    use super::*;

    pub fn list(ctx: Context<List>, prices: Vec<u64>) -> Result<()> {
        let mut min = u64::MAX;
        for &p in prices.iter() {
            // 単一条件の if
            if p < min {
                min = p;
            }
        }
        let l = &mut ctx.accounts.listing;
        l.lowest = min;
        Ok(())
    }

    pub fn buy(ctx: Context<Buy>, qty: u32) -> Result<()> {
        let _l = &ctx.accounts.listing;
        let pur = &mut ctx.accounts.purchase_account;
        pur.buyer = ctx.accounts.user.key();
        pur.qty = qty;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct List<'info> {
    #[account(init, payer = seller, space = 8 + 8)]
    pub listing: Account<'info, ListingData>,
    #[account(mut)] pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Buy<'info> {
    pub listing: Account<'info, ListingData>,
    #[account(mut, init, payer = user, space = 8 + 32 + 4)]
    pub purchase_account: Account<'info, PurchaseData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ListingData {
    pub lowest: u64,
}

#[account]
pub struct PurchaseData {
    pub buyer: Pubkey,
    pub qty: u32,
}
