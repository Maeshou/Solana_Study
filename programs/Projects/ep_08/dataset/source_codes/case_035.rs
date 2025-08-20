use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

declare_id!("DIV035035035035035035035035035035");

#[program]
pub mod case_035 {
    use super::*;

    pub fn configure(ctx: Context<Ctx035>, bump: u8) -> Result<()> {
        let seed = &[b"alpha035", bump.to_le_bytes().as_ref()];
        let (pda, _) = Pubkey::find_program_address(seed, ctx.program_id);
        ctx.accounts.storage.account = pda;
        ctx.accounts.storage.version = bump;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Ctx035<'info> {
    #[account(mut)] pub admin: Signer<'info>,
    #[account(init, payer = admin, seeds = [b"alpha035", bump.to_le_bytes().as_ref()], bump)]
    pub storage: Account<'info, Storage035>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Storage035 {
    pub account: Pubkey,
    pub version: u8,
}
