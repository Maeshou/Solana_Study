use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};

declare_id!("GuIlDrAnKc0sp1ay1111111111111111111111");

#[program]
pub mod guild_rank_gate {
    use super::*;
    pub fn promote(ctx: Context<Promote>, level: u8) -> Result<()> {
        if ctx.accounts.cfg.owner != crate::ID {
            return Err(ProgramError::IllegalOwner.into());
        }
        let rank = GuildRank::try_from_slice(&ctx.accounts.cfg.data.borrow())?;
        require_keys_eq!(rank.master, ctx.accounts.signer.key(), ProgramError::MissingRequiredSignature);
        msg!("Promoting to level {}", level);
        Ok(())
    }
    pub fn write_badge(ctx: Context<WriteBadge>, key: Pubkey) -> Result<()> {
        let badge = MemberBadge { holder: key };
        ctx.accounts.cfg.data.borrow_mut()[..32].copy_from_slice(&badge.try_to_vec()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Promote<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
}
#[derive(Accounts)]
pub struct WriteBadge<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct GuildRank { pub master: Pubkey }
#[derive(BorshSerialize, BorshDeserialize)]
pub struct MemberBadge { pub holder: Pubkey }
