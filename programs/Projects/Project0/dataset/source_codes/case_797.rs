use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf797mvTWf");

#[program]
pub mod register_data_797 {
    use super::*;

    pub fn register_data(ctx: Context<RegisterData797>, info: String) -> Result<()> {
        let md_bump = *ctx.bumps.get("metadata").unwrap();
        let md = &mut ctx.accounts.metadata;
        md.bump = md_bump;
        md.creator = ctx.accounts.user.key();
        md.info = info.clone();
        md.length = md.info.len() as u64;
        msg!(
            "Case 797: bump={} creator={} length={}",
            md_bump,
            md.creator,
            md.length
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterData797<'info> {
    #[account(init, seeds = [b"metadata", user.key().as_ref()], bump,
       payer = user, space = 8 + 1 + 32 + 4 + 128 + 8)]
    pub metadata: Account<'info, Metadata797>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Metadata797 {
    pub bump: u8,
    pub creator: Pubkey,
    pub info: String,
    pub length: u64,
}
