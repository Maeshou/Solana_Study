use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA06mvTWf");

#[program]
pub mod pubkey_digest_store_003 {
    use super::*;

    pub fn save_key_digest(ctx: Context<Ctx003>) -> Result<()> {
        let pubkey_bytes = ctx.accounts.authority.key().to_bytes();
        let sliced = [
            pubkey_bytes[0],
            pubkey_bytes[1],
            pubkey_bytes[2],
            pubkey_bytes[3],
            pubkey_bytes[4],
            pubkey_bytes[5],
            pubkey_bytes[6],
            pubkey_bytes[7],
        ];
        let numeric = u64::from_le_bytes(sliced);
        ctx.accounts.storage.data = numeric;
        Ok(())
    }

    pub fn view(ctx: Context<Ctx003>) -> Result<()> {
        msg!("Stored numeric digest: {}", ctx.accounts.storage.data);
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
    pub data: u64,
}
