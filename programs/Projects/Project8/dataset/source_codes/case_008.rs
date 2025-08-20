// Program 7: arena_score_log (固定IDに対するInstruction; PDA署名)
// ここではSystemProgram.Transferと組み合わせて二段階処理
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed, system_instruction};

declare_id!("ArenaScoreLog777777777777777777777777777");
const MEMO_ID: Pubkey = pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

#[program]
pub mod arena_score_log {
    use super::*;

    pub fn init_arena(ctx: Context<InitArena>, season: u64) -> Result<()> {
        let a = &mut ctx.accounts.arena;
        a.owner = ctx.accounts.owner.key();
        a.season = season.rotate_left(1).wrapping_add(15);
        Ok(())
    }

    pub fn payout_and_log(ctx: Context<PayoutAndLog>, lamports: u64, note: Vec<u8>) -> Result<()> {
        // 1) payout
        let transfer_ix = system_instruction::transfer(&ctx.accounts.arena.key(), &ctx.accounts.player.key(), lamports);
        let bump = *ctx.bumps.get("arena").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"arena", ctx.accounts.owner.key.as_ref(), &ctx.accounts.arena.season.to_le_bytes(), &[bump]];
        invoke_signed(
            &transfer_ix,
            &[
                ctx.accounts.arena.to_account_info(),
                ctx.accounts.player.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        // 2) memo
        let memo_ix = Instruction { program_id: MEMO_ID, accounts: vec![], data: note };
        invoke_signed(&memo_ix, &[ctx.accounts.arena.to_account_info()], &[seeds])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitArena<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8,
        seeds=[b"arena", owner.key().as_ref(), season.to_le_bytes().as_ref()],
        bump
    )]
    pub arena: Account<'info, Arena>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub season: u64,
}

#[derive(Accounts)]
pub struct PayoutAndLog<'info> {
    #[account(
        mut,
        seeds=[b"arena", owner.key().as_ref(), arena.season.to_le_bytes().as_ref()],
        bump
    )]
    pub arena: Account<'info, Arena>,
    #[account(mut)]
    pub player: SystemAccount<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Arena {
    pub owner: Pubkey,
    pub season: u64,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
