use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf991mvTWf");

#[program]
pub mod analyze_pubkey_length_003 {
    use super::*;

    pub fn store_pubkey_length(ctx: Context<Ctx003>, threshold: u64) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);

        let key_str = ctx.accounts.authority.key().to_string();
        let len = key_str.len() as u64;
        ctx.accounts.storage.data = if len >= threshold { 1 } else { 0 };

        msg!(
            "Pubkey length = {} (threshold = {}), Flag set to {}",
            len,
            threshold,
            ctx.accounts.storage.data
        );
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
    pub data: u64, // 1 = 長さ超え, 0 = 未満
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access")]
    Unauthorized,
}
