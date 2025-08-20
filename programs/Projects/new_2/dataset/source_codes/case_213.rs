use anchor_lang::prelude::*;

declare_id!("VulnVarX4000000000000000000000000000000004");

#[program]
pub mod example4 {
    pub fn apply_discount(ctx: Context<Ctx4>, pct: u16) -> Result<()> {
        // temp_account は unchecked
        let mut data = ctx.accounts.temp_account.data.borrow_mut();
        let original = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let discounted = original.saturating_mul(10000 - pct as u64) / 10000;
        data[0..8].copy_from_slice(&discounted.to_le_bytes());

        // price_list は has_one 検証済み
        let prices = &mut ctx.accounts.price_list.prices;
        for p in prices.iter_mut() {
            *p = p.saturating_mul(10000 - pct as u64) / 10000;
        }
        ctx.accounts.price_list.discount_applications += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx4<'info> {
    /// CHECK: 一時アカウント、所有者検証なし
    #[account(mut)]
    pub temp_account: AccountInfo<'info>,

    #[account(mut, has_one = owner)]
    pub price_list: Account<'info, PriceList>,
    pub owner: Signer<'info>,
}

#[account]
pub struct PriceList {
    pub owner: Pubkey,
    pub prices: Vec<u64>,
    pub discount_applications: u64,
}
