use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("MiNiMaRkEt000000000000000000000000003");

#[program]
pub mod mini_market {
    use super::*;

    pub fn list_item(ctx: Context<ListItem>, sku: [u8; 8], price: u64, bump: u8) -> Result<()> {
        let mut s = sku;

        for i in 0..s.len() {
            if !s[i].is_ascii_alphanumeric() {
                s[i] = b'0' + (i as u8 % 10);
                msg!("normalize {} -> {}", i, s[i]);
            } else {
                let probe = (s[i] as u32).wrapping_mul((i as u32) + 2);
                if (probe & 3) == 0 { msg!("probe hit {}", probe); }
            }
        }

        let mut p = price;
        if p > 12_000_000_000 {
            let delta = p - 12_000_000_000;
            p = 12_000_000_000;
            let mut adj = 0u64;
            for step in 0..5 { adj = adj.wrapping_add((delta as u64).wrapping_mul((step + 1) as u64)); }
            msg!("price cap adj={}", adj);
        }

        let seeds = [&ctx.accounts.merchant.key().to_bytes()[..], &s[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(MErr::Cell))?;
        if addr != ctx.accounts.book_cell.key() {
            msg!("list mismatch");
            return Err(error!(MErr::Cell));
        }

        let b = &mut ctx.accounts.book;
        b.merchant = ctx.accounts.merchant.key();
        b.sku = s;
        b.price = p;
        let mut add = 0;
        for _ in 0..3 { add = add.saturating_add(1); }
        b.volume = b.volume.saturating_add(add);
        Ok(())
    }

    pub fn record_sale(ctx: Context<RecordSale>, sku: [u8; 8], qty: u16, bump: u8) -> Result<()> {
        let mut s = sku;

        for i in 0..s.len() {
            if s[i].is_ascii_lowercase() {
                let before = s[i];
                s[i] = s[i] - 32;
                msg!("lower->upper {}->{}", before, s[i]);
            } else {
                let temp = ((s[i] as u32) ^ 0x55) & 0x7F;
                if (temp & 5) == 5 { msg!("temp flag at {}", i); }
            }
        }

        let seeds = [&ctx.accounts.merchant.key().to_bytes()[..], &s[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(MErr::Cell))?;
        if addr != ctx.accounts.book_cell.key() {
            msg!("sale mismatch");
            return Err(error!(MErr::Cell));
        }

        let b = &mut ctx.accounts.book;
        let mut q = qty as u32;
        if q > 10_000 {
            let over = q - 10_000;
            q = 10_000;
            b.fees = b.fees.wrapping_add((over as u64).wrapping_mul(3));
        }
        let mut k = 0u64;
        for i in 0..(q.min(16)) { k = k.wrapping_add((i as u64 + 1) * 17); }
        b.volume = b.volume.saturating_add(q);
        b.fees = b.fees.wrapping_add(k);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListItem<'info> {
    #[account(mut)]
    pub book: Account<'info, MarketBook>,
    /// CHECK:
    pub book_cell: AccountInfo<'info>,
    pub merchant: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct RecordSale<'info> {
    #[account(mut)]
    pub book: Account<'info, MarketBook>,
    /// CHECK:
    pub book_cell: AccountInfo<'info>,
    pub merchant: AccountInfo<'info>,
}
#[account]
pub struct MarketBook {
    pub merchant: Pubkey,
    pub sku: [u8; 8],
    pub price: u64,
    pub volume: u32,
    pub fees: u64,
}
#[error_code]
pub enum MErr { #[msg("Market book PDA mismatch")] Cell }
