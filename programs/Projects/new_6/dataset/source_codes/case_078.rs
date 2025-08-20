use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};

declare_id!("RaIdLoGc0sp1ay111111111111111111111111");

#[program]
pub mod raid_log_gate {
    use super::*;
    pub fn record(ctx: Context<Record>, score: u32) -> Result<()> {
        if ctx.accounts.cfg.owner != crate::ID {
            return Err(ProgramError::IllegalOwner.into());
        }
        let log = RaidLog::try_from_slice(&ctx.accounts.cfg.data.borrow())?;
        require_keys_eq!(log.logger, ctx.accounts.signer.key(), ProgramError::MissingRequiredSignature);
        msg!("Recording raid score {}", score);
        Ok(())
    }
    pub fn write_stats(ctx: Context<WriteStats>, key: Pubkey) -> Result<()> {
        let stats = PlayerStats { player: key };
        ctx.accounts.cfg.data.borrow_mut()[..32].copy_from_slice(&stats.try_to_vec()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Record<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
}
#[derive(Accounts)]
pub struct WriteStats<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct RaidLog { pub logger: Pubkey }
#[derive(BorshSerialize, BorshDeserialize)]
pub struct PlayerStats { pub player: Pubkey }
