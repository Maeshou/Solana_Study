use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA16mvTWf");

#[program]
pub mod median_computer_003 {
    use super::*;

    pub fn store_and_compute(ctx: Context<Ctx003>, a: u64, b: u64, c: u64) -> Result<()> {
        // 最大 + 最小 - 合計 = 中央値 × 2 → 中央値 = a + b + c - max - min
        let sum = a + b + c;
        let max_ab = (a > b) as u64 * a + (a <= b) as u64 * b;
        let max = (max_ab > c) as u64 * max_ab + (max_ab <= c) as u64 * c;

        let min_ab = (a < b) as u64 * a + (a >= b) as u64 * b;
        let min = (min_ab < c) as u64 * min_ab + (min_ab >= c) as u64 * c;

        let median = sum - max - min;

        let s = &mut ctx.accounts.storage;
        s.val1 = a;
        s.val2 = b;
        s.val3 = c;
        s.median_value = median;

        Ok(())
    }

    pub fn show(ctx: Context<Ctx003>) -> Result<()> {
        let s = &ctx.accounts.storage;
        msg!("Values: {}, {}, {}", s.val1, s.val2, s.val3);
        msg!("Median: {}", s.median_value);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub val1: u64,
    pub val2: u64,
    pub val3: u64,
    pub median_value: u64,
}
