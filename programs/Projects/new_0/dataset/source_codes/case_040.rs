use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA28mvTWf");

#[program]
pub mod identity_encoder_003 {
    use super::*;

    pub fn encode_identity(ctx: Context<Ctx003>) -> Result<()> {
        let bytes = ctx.accounts.authority.key().to_bytes();
        let encoded = u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        ctx.accounts.storage.identity_hash = encoded;
        Ok(())
    }

    pub fn show(ctx: Context<Ctx003>) -> Result<()> {
        msg!("Identity Encoded: {}", ctx.accounts.storage.identity_hash);
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
    pub identity_hash: u64,
}
