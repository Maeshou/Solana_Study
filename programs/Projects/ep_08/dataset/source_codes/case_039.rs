use anchor_lang::prelude::*;
use sha2::Sha256;
use sha2::Digest;

declare_id!("DIV039039039039039039039039039039");

#[program]
pub mod case_039 {
    use super::*;

    pub fn hash_bump(ctx: Context<Hash039>, bump: u8) -> Result<()> {
        let mut hasher = Sha256::new();
        hasher.update(&[bump]);
        let result = hasher.finalize();
        ctx.accounts.data.hash = result.to_vec();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Hash039<'info> {
    #[account(init, payer = user, seeds = [b"epsilon039", bump.to_le_bytes().as_ref()], bump)]
    pub data: Account<'info, Data039>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Data039 {
    pub hash: Vec<u8>,
}
