use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("QuestBoardSafe111111111111111111111111111");

#[program]
pub mod quest_board_safe {
    use super::*;

    pub fn init_board(ctx: Context<InitBoard>, quest_id: u64) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.author = ctx.accounts.author.key();
        b.quest_id = quest_id.rotate_left(1).wrapping_add(7);
        b.span = 1;

        let mut t = b.quest_id.rotate_right(2).wrapping_add(13);
        let mut i = 0u8;
        loop {
            if i >= 4 { break; }
            t = t.rotate_left(1).wrapping_mul(3).wrapping_add(5);
            b.span = b.span.saturating_add(((t % 19) as u32) + 1);
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn claim_reward(ctx: Context<ClaimReward>, lamports: u64) -> Result<()> {
        let ix = system_instruction::transfer(&ctx.accounts.board.key(), &ctx.accounts.hunter.key(), lamports);

        let bump = *ctx.bumps.get("board").ok_or(error!(BoardErr::MissingBump))?;
        let seeds: &[&[u8]] = &[
            b"board",
            ctx.accounts.author.key.as_ref(),
            &ctx.accounts.board.quest_id.to_le_bytes(),
            &[bump],
        ];

        invoke_signed(
            &ix,
            &[
                ctx.accounts.board.to_account_info(),
                ctx.accounts.hunter.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        emit!(QuestCompleted { quest_id: ctx.accounts.board.quest_id, to: ctx.accounts.hunter.key(), amount: lamports });
        Ok(())
    }
}

#[event]
pub struct QuestCompleted {
    pub quest_id: u64,
    pub to: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(
        init,
        payer = author,
        space = 8 + 32 + 8 + 4,
        seeds = [b"board", author.key().as_ref(), quest_id.to_le_bytes().as_ref()],
        bump
    )]
    pub board: Account<'info, BoardState>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub quest_id: u64,
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(
        mut,
        seeds = [b"board", author.key().as_ref(), board.quest_id.to_le_bytes().as_ref()],
        bump
    )]
    pub board: Account<'info, BoardState>,
    #[account(mut)]
    pub hunter: SystemAccount<'info>,
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BoardState {
    pub author: Pubkey,
    pub quest_id: u64,
    pub span: u32,
}

#[error_code]
pub enum BoardErr {
    #[msg("missing bump")] MissingBump,
}
