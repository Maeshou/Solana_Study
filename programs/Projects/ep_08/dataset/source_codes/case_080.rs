use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

declare_id!("DIV080080080080080080080080080080");

#[program]
pub mod case_080 {
    use super::*;

    pub fn configure(ctx: Context<Ctx080>, bump: u8) -> Result<()> {
        let seed = &[b"alpha080", bump.to_le_bytes().as_ref()];
        let (pda, _) = Pubkey::find_program_address(seed, ctx.program_id);
        ctx.accounts.storage.account = pda;
        ctx.accounts.storage.version = bump;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Ctx080<'info> {
    #[account(mut)] pub admin: Signer<'info>,
    #[account(init, payer = admin, seeds = [b"alpha080", bump.to_le_bytes().as_ref()], bump)]
    pub storage: Account<'info, Storage080>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Storage080 {
    pub account: Pubkey,
    pub version: u8,
}
