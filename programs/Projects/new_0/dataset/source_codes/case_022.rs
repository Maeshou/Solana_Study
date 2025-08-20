use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA08mvTWf");

#[program]
pub mod float_encode_store_003 {
    use super::*;

    pub fn store_float(ctx: Context<Ctx003>, float_val: f64) -> Result<()> {
        let encoded = float_val.to_bits(); // f64 → u64 のビット変換
        ctx.accounts.storage.data = encoded;
        Ok(())
    }

    pub fn read_float(ctx: Context<Ctx003>) -> Result<()> {
        let raw = ctx.accounts.storage.data;
        let decoded = f64::from_bits(raw); // u64 → f64 のビット復元
        msg!("Stored float value: {}", decoded);
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
    pub data: u64, // f64のビット列として格納
}
