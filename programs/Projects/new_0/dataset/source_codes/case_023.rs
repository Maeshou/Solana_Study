use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA09mvTWf");

#[program]
pub mod float_array_summary_003 {
    use super::*;

    // 入力された3つの浮動小数点値を格納し、合計値も別途保存
    pub fn store_floats(ctx: Context<Ctx003>, val1: f64, val2: f64, val3: f64) -> Result<()> {
        let bits1 = val1.to_bits();
        let bits2 = val2.to_bits();
        let bits3 = val3.to_bits();

        let sum = val1 + val2 + val3;
        let sum_bits = sum.to_bits();

        let store = &mut ctx.accounts.storage;
        store.data1 = bits1;
        store.data2 = bits2;
        store.data3 = bits3;
        store.total = sum_bits;

        Ok(())
    }

    // 保存された合計と平均を表示
    pub fn read_summary(ctx: Context<Ctx003>) -> Result<()> {
        let store = &ctx.accounts.storage;

        let v1 = f64::from_bits(store.data1);
        let v2 = f64::from_bits(store.data2);
        let v3 = f64::from_bits(store.data3);
        let sum = f64::from_bits(store.total);
        let average = (v1 + v2 + v3) / 3.0;

        msg!("Stored values: {}, {}, {}", v1, v2, v3);
        msg!("Sum: {}", sum);
        msg!("Average (recomputed): {}", average);
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
    pub data1: u64,
    pub data2: u64,
    pub data3: u64,
    pub total: u64,
}
