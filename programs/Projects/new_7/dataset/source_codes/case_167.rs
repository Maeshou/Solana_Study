use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Mix02DripNotify11111111111111111111111111");

const STATS_ID_B: Pubkey = pubkey!("StAtSBBBBBBBBB111111111111111111111111111");

#[program]
pub mod drip_and_notify_mix {
    use super::*;

    pub fn act(ctx: Context<Act>, ticks: u64, prize: u64) -> Result<()> {
        if ticks > 5 {
            ctx.accounts.note.bump = ctx.accounts.note.bump.wrapping_add(1);
        }
        if prize > 0 {
            ctx.accounts.note.total = ctx.accounts.note.total.saturating_add(prize);
        }

        // 固定ID CPI（安全寄り）
        let fixed_ix = Instruction {
            program_id: STATS_ID_B,
            accounts: vec![
                AccountMeta::new(ctx.accounts.stats_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.participant.key(), false),
            ],
            data: ticks.to_le_bytes().to_vec(),
        };
        invoke(&fixed_ix, &[
            ctx.accounts.stats_marker.to_account_info(),
            ctx.accounts.stats_cell.to_account_info(),
            ctx.accounts.participant.to_account_info(),
        ])?;

        // 動的CPI
        let mut program_ai = ctx.accounts.notify_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            program_ai = ctx.remaining_accounts[0].clone();
            ctx.accounts.note.paths = ctx.accounts.note.paths.saturating_add(2);
        }
        let dyn_ix = Instruction {
            program_id: *program_ai.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.notify_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.participant.key(), false),
            ],
            data: prize.to_le_bytes().to_vec(),
        };
        invoke(&dyn_ix, &[
            program_ai,
            ctx.accounts.notify_board.to_account_info(),
            ctx.accounts.participant.to_account_info(),
        ])?;

        // SPL Token（安全寄り）
        let t = Transfer {
            from: ctx.accounts.prize_pool.to_account_info(),
            to: ctx.accounts.user_token.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info(),
        };
        let tctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), t);
        token::transfer(tctx, prize)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Act<'info> {
    #[account(mut)]
    pub note: Account<'info, DripNote>,
    /// CHECK:
    pub stats_cell: AccountInfo<'info>,
    /// CHECK:
    pub participant: AccountInfo<'info>,
    /// CHECK:
    pub stats_marker: AccountInfo<'info>,
    /// CHECK:
    pub notify_board: AccountInfo<'info>,
    /// CHECK:
    pub notify_hint: AccountInfo<'info>,
    #[account(mut)]
    pub prize_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub pool_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct DripNote {
    pub bump: u64,
    pub total: u64,
    pub paths: u64,
}
