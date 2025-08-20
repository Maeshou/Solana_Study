use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA36mvTWf");

#[program]
pub mod nft_rent_register_003 {
    use super::*;

    pub fn register_rental(ctx: Context<Ctx003>, nft_id: u64, start_slot: u64) -> Result<()> {
        let r = &mut ctx.accounts.rent_info;
        r.owner = ctx.accounts.owner.key();
        r.nft_id = nft_id;
        r.start_slot = start_slot;
        Ok(())
    }

    pub fn show(ctx: Context<Ctx003>) -> Result<()> {
        let r = &ctx.accounts.rent_info;
        msg!("NFT ID: {}", r.nft_id);
        msg!("Start Slot: {}", r.start_slot);
        msg!("Owner: {}", r.owner);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(init_if_needed, payer = owner, space = 8 + 32 + 8 + 8, seeds = [b"rent", owner.key().as_ref()], bump)]
    pub rent_info: Account<'info, RentInfo>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RentInfo {
    pub owner: Pubkey,
    pub nft_id: u64,
    pub start_slot: u64,
}
