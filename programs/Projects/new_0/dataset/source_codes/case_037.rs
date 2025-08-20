use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA24mvTWf");

#[program]
pub mod identity_hasher_003 {
    use super::*;

    pub fn store_identity_score(ctx: Context<Ctx003>) -> Result<()> {
        let bytes = ctx.accounts.authority.key().to_bytes();
        let mut total: u64 = 0;

        // バイト合計（ループなしで明示的に）
        total += bytes[0] as u64;
        total += bytes[1] as u64;
        total += bytes[2] as u64;
        total += bytes[3] as u64;
        total += bytes[4] as u64;
        total += bytes[5] as u64;
        total += bytes[6] as u64;
        total += bytes[7] as u64;
        total += bytes[8] as u64;
        total += bytes[9] as u64;
        total += bytes[10] as u64;
        total += bytes[11] as u64;
        total += bytes[12] as u64;
        total += bytes[13] as u64;
        total += bytes[14] as u64;
        total += bytes[15] as u64;
        total += bytes[16] as u64;
        total += bytes[17] as u64;
        total += bytes[18] as u64;
        total += bytes[19] as u64;
        total += bytes[20] as u64;
        total += bytes[21] as u64;
        total += bytes[22] as u64;
        total += bytes[23] as u64;
        total += bytes[24] as u64;
        total += bytes[25] as u64;
        total += bytes[26] as u64;
        total += bytes[27] as u64;
        total += bytes[28] as u64;
        total += bytes[29] as u64;
        total += bytes[30] as u64;
        total += bytes[31] as u64;

        ctx.accounts.storage.identity_score = total;
        Ok(())
    }

    pub fn show(ctx: Context<Ctx003>) -> Result<()> {
        msg!("Identity Score: {}", ctx.accounts.storage.identity_score);
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
    pub identity_score: u64,
}
