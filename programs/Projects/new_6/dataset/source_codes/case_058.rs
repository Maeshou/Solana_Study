use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};

declare_id!("QuEsTb0ArDc0sp1ay111111111111111111111");

#[program]
pub mod quest_board_gate {
    use super::*;
    pub fn post_quest(ctx: Context<PostQuest>, quest_id: u32) -> Result<()> {
        if ctx.accounts.cfg.owner != crate::ID {
            return Err(ProgramError::IllegalOwner.into());
        }
        let board = QuestBoard::try_from_slice(&ctx.accounts.cfg.data.borrow())?;
        require_keys_eq!(board.admin, ctx.accounts.signer.key(), ProgramError::MissingRequiredSignature);
        msg!("Posting quest id {}", quest_id);
        Ok(())
    }
    pub fn write_adventurer(ctx: Context<WriteAdventurer>, key: Pubkey) -> Result<()> {
        let card = AdventurerCard { adventurer: key };
        ctx.accounts.cfg.data.borrow_mut()[..32].copy_from_slice(&card.try_to_vec()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PostQuest<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
}
#[derive(Accounts)]
pub struct WriteAdventurer<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct QuestBoard { pub admin: Pubkey }
#[derive(BorshSerialize, BorshDeserialize)]
pub struct AdventurerCard { pub adventurer: Pubkey }
