use anchor_lang::prelude::*;
use sha2::{Digest, Sha256};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA14mvTWf");

#[program]
pub mod hashed_auth_writer_003 {
    use super::*;

    pub fn write_if_hashed_key_matches(ctx: Context<Ctx003>, new_value: u64) -> Result<()> {
        let signer_pubkey = ctx.accounts.authority.key().to_bytes();
        let computed_hash = Sha256::digest(&signer_pubkey);
        let stored_hash = ctx.accounts.storage.hash_key;

        // 一致していれば更新（分岐なし）
        let is_match = computed_hash[..] == stored_hash[..];
        let allow = is_match as u8;

        // 書き込み処理（許可時のみ）
        let current = ctx.accounts.storage.data;
        let new = allow * new_value as u8 + (1 - allow) * current as u8;
        ctx.accounts.storage.data = new as u64;

        Ok(())
    }

    pub fn read(ctx: Context<Ctx003>) -> Result<()> {
        msg!("Stored data: {}", ctx.accounts.storage.data);
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
    pub hash_key: [u8; 32], // SHA-256で事前に設定されたPubkeyのハッシュ
    pub data: u64,
}
