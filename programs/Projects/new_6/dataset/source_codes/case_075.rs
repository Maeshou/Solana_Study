use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};

declare_id!("EnErGyForGeC0sp1ay111111111111111111111");

#[program]
pub mod forge_access {
    use super::*;
    pub fn refuel(ctx: Context<Refuel>, energy: u16) -> Result<()> {
        if ctx.accounts.cfg.owner != crate::ID {
            return Err(ProgramError::IllegalOwner.into());
        }
        let forge = EnergyForge::try_from_slice(&ctx.accounts.cfg.data.borrow())?;
        require_keys_eq!(forge.owner, ctx.accounts.signer.key(), ProgramError::MissingRequiredSignature);
        msg!("Refueled {} units", energy);
        Ok(())
    }
    pub fn write_blacksmith(ctx: Context<WriteBlacksmith>, key: Pubkey) -> Result<()> {
        let profile = BlacksmithProfile { smith: key };
        ctx.accounts.cfg.data.borrow_mut()[..32].copy_from_slice(&profile.try_to_vec()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Refuel<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
}
#[derive(Accounts)]
pub struct WriteBlacksmith<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct EnergyForge { pub owner: Pubkey }
#[derive(BorshSerialize, BorshDeserialize)]
pub struct BlacksmithProfile { pub smith: Pubkey }
