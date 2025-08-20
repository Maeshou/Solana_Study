use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};

declare_id!("NftStAti0nC0sp1ay1111111111111111111111");

#[program]
pub mod nft_station_access {
    use super::*;
    pub fn craft(ctx: Context<Craft>, seed: u64) -> Result<()> {
        if ctx.accounts.cfg.owner != crate::ID {
            return Err(ProgramError::IllegalOwner.into());
        }
        let station = NftCraftStation::try_from_slice(&ctx.accounts.cfg.data.borrow())?;
        require_keys_eq!(station.creator, ctx.accounts.signer.key(), ProgramError::MissingRequiredSignature);
        msg!("Crafting NFT with seed {}", seed);
        Ok(())
    }
    pub fn write_profile(ctx: Context<WriteProfile>, key: Pubkey) -> Result<()> {
        let profile = CrafterProfile { player: key };
        ctx.accounts.cfg.data.borrow_mut()[..32].copy_from_slice(&profile.try_to_vec()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Craft<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
}
#[derive(Accounts)]
pub struct WriteProfile<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct NftCraftStation { pub creator: Pubkey }
#[derive(BorshSerialize, BorshDeserialize)]
pub struct CrafterProfile { pub player: Pubkey }
