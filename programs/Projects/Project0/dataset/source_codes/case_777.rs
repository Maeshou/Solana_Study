use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf777mvTWf");

#[program]
pub mod register_data_777 {
    use super::*;

    pub fn register_data(ctx: Context<RegisterData777>, info: String) -> Result<()> {
        let md_bump = *ctx.bumps.get("metadata").unwrap();
        let md = &mut ctx.accounts.metadata;
        md.bump = md_bump;
        md.creator = ctx.accounts.user.key();
        md.info = info.clone();
        md.length = md.info.len() as u64;
        msg!(
            "Case 777: bump={} creator={} length={}",
            md_bump,
            md.creator,
            md.length
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterData777<'info> {
    #[account(init, seeds = [b"metadata", user.key().as_ref()], bump,
       payer = user, space = 8 + 1 + 32 + 4 + 128 + 8)]
    pub metadata: Account<'info, Metadata777>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Metadata777 {
    pub bump: u8,
    pub creator: Pubkey,
    pub info: String,
    pub length: u64,
}
