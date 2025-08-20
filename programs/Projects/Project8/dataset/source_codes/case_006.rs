// Program 4: quest_reward_memo (MemoプログラムID固定でinvoke; PDA署名)
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed, system_program};

declare_id!("Qu3stRewardMemo4444444444444444444444444");
const MEMO_ID: Pubkey = pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

#[program]
pub mod quest_reward_memo {
    use super::*;

    pub fn init_board(ctx: Context<InitBoard>, nonce: u64) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.owner = ctx.accounts.owner.key();
        b.nonce = nonce.rotate_left(2).wrapping_add(5);
        b.weight = (b.nonce ^ 31).rotate_right(1);
        Ok(())
    }

    pub fn write_memo(ctx: Context<WriteMemo>, content: Vec<u8>) -> Result<()> {
        let accounts = vec![];
        let ix = Instruction { program_id: MEMO_ID, accounts, data: content };

        let bump = *ctx.bumps.get("board").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[
            b"board",
            ctx.accounts.owner.key.as_ref(),
            &ctx.accounts.board.nonce.to_le_bytes(),
            &[bump],
        ];

        invoke_signed(&ix, &[ctx.accounts.board.to_account_info()], &[seeds])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 8,
        seeds=[b"board", owner.key().as_ref(), nonce.to_le_bytes().as_ref()],
        bump
    )]
    pub board: Account<'info, Board>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub nonce: u64,
}

#[derive(Accounts)]
pub struct WriteMemo<'info> {
    #[account(
        mut,
        seeds=[b"board", owner.key().as_ref(), board.nonce.to_le_bytes().as_ref()],
        bump
    )]
    pub board: Account<'info, Board>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Board {
    pub owner: Pubkey,
    pub nonce: u64,
    pub weight: u64,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
