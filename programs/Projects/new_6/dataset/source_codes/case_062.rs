use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};

declare_id!("MaRkEtLiStC0sp1ay11111111111111111111");

#[program]
pub mod market_list_gate {
    use super::*;
    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        if ctx.accounts.cfg.owner != crate::ID {
            return Err(ProgramError::IllegalOwner.into());
        }
        let listing = MarketListing::try_from_slice(&ctx.accounts.cfg.data.borrow())?;
        require_keys_eq!(listing.owner, ctx.accounts.signer.key(), ProgramError::MissingRequiredSignature);
        msg!("Listing item for {} lamports", price);
        Ok(())
    }
    pub fn write_seller(ctx: Context<WriteSeller>, key: Pubkey) -> Result<()> {
        let seller = SellerProfile { seller: key };
        ctx.accounts.cfg.data.borrow_mut()[..32].copy_from_slice(&seller.try_to_vec()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
}
#[derive(Accounts)]
pub struct WriteSeller<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MarketListing { pub owner: Pubkey }
#[derive(BorshSerialize, BorshDeserialize)]
pub struct SellerProfile { pub seller: Pubkey }
