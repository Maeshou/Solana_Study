use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};

declare_id!("ArEnAmAnAgEc0sp1ay1111111111111111111");

#[program]
pub mod arena_gate {
    use super::*;
    pub fn start_match(ctx: Context<StartMatch>, opponent: Pubkey) -> Result<()> {
        if ctx.accounts.cfg.owner != crate::ID {
            return Err(ProgramError::IllegalOwner.into());
        }
        let manager = ArenaManager::try_from_slice(&ctx.accounts.cfg.data.borrow())?;
        require_keys_eq!(manager.judge, ctx.accounts.signer.key(), ProgramError::MissingRequiredSignature);
        msg!("Starting match against {:?}", opponent);
        Ok(())
    }
    pub fn write_fighter(ctx: Context<WriteFighter>, key: Pubkey) -> Result<()> {
        let fighter = FighterCard { fighter: key };
        ctx.accounts.cfg.data.borrow_mut()[..32].copy_from_slice(&fighter.try_to_vec()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartMatch<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
}
#[derive(Accounts)]
pub struct WriteFighter<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ArenaManager { pub judge: Pubkey }
#[derive(BorshSerialize, BorshDeserialize)]
pub struct FighterCard { pub fighter: Pubkey }
