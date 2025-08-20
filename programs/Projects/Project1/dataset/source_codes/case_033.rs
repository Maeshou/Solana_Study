use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA19mvTWf");

#[program]
pub mod serial_issuer_003 {
    use super::*;

    pub fn increment_counter(ctx: Context<Ctx003>) -> Result<()> {
        let current = ctx.accounts.storage.last_issued;
        let next = current + 1;
        ctx.accounts.storage.last_issued = next;
        Ok(())
    }

    pub fn show(ctx: Context<Ctx003>) -> Result<()> {
        let s = &ctx.accounts.storage;
        msg!("Last Issued Serial: {}", s.last_issued);
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
    pub last_issued: u64,
}
