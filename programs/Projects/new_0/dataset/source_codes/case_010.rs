use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf010mvTWf");

#[program]
pub mod authorize_authority_010 {
    use super::*;

    pub fn authorize_authority(ctx: Context<Ctx010>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 010: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx010<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry010>,use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;
use sha2::{Sha256, Digest};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf992mvTWf");

#[program]
pub mod pseudo_randomize_003 {
    use super::*;

    pub fn generate_pseudo_random(ctx: Context<Ctx003>) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);

        let now = Clock::get()?.unix_timestamp;
        let pubkey_bytes = ctx.accounts.authority.key().to_bytes();
        let mut hasher = Sha256::new();
        hasher.update(&pubkey_bytes);
        hasher.update(&now.to_le_bytes());
        let result = hasher.finalize();

        // 最初の8バイトを u64 に変換して記録
        let pseudo_random = u64::from_le_bytes(result[..8].try_into().unwrap());
        ctx.accounts.storage.data = pseudo_random;

        msg!("Generated pseudo-random value: {}", pseudo_random);
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

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access")]
    Unauthorized,
}

    #[account(signer)]
    pub admin: Signer<'info>,
    pub caller: Signer<'info>,
}

#[account]
pub struct Entry010 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
