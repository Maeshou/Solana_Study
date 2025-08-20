use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA21mvTWf");

#[program]
pub mod multiplier_register_003 {
    use super::*;

    pub fn multiply_and_store(ctx: Context<Ctx003>, a: u64, b: u64) -> Result<()> {
        let result = a.wrapping_mul(b); // u64のオーバーフロー対策
        ctx.accounts.storage.product = result;
        ctx.accounts.storage.last_a = a;
        ctx.accounts.storage.last_b = b;
        Ok(())
    }

    pub fn read(ctx: Context<Ctx003>) -> Result<()> {
        let s = &ctx.accounts.storage;
        msg!("a: {}", s.last_a);
        msg!("b: {}", s.last_b);
        msg!("Product: {}", s.product);
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
    pub last_a: u64,
    pub last_b: u64,
    pub product: u64,
}
