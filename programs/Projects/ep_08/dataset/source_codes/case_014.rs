use anchor_lang::prelude::*;
use sha2::Sha256;
use sha2::Digest;

declare_id!("DIV014014014014014014014014014014");

#[program]
pub mod case_014 {
    use super::*;

    pub fn hash_bump(ctx: Context<Hash014>, bump: u8) -> Result<()> {
        let mut hasher = Sha256::new();
        hasher.update(&[bump]);
        let result = hasher.finalize();
        ctx.accounts.data.hash = result.to_vec();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Hash014<'info> {
    #[account(init, payer = user, seeds = [b"epsilon014", bump.to_le_bytes().as_ref()], bump)]
    pub data: Account<'info, Data014>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Data014 {
    pub hash: Vec<u8>,
}
