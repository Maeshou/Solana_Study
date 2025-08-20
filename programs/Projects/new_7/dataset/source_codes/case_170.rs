use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Mix05GradeSignal1111111111111111111111111");

const FIXED_GAUGE: Pubkey = pubkey!("FiXeDgAuGeEEEE11111111111111111111111111");

#[program]
pub mod grade_and_signal_mix {
    use super::*;

    pub fn submit(ctx: Context<Submit>, score: u64, gift: u64) -> Result<()> {
        if score > 90 { ctx.accounts.journal.stars = ctx.accounts.journal.stars.saturating_add(1); }
        if score < 50 { ctx.accounts.journal.warnings = ctx.accounts.journal.warnings.wrapping_add(1); }

        let fixed_ix = Instruction {
            program_id: FIXED_GAUGE,
            accounts: vec![
                AccountMeta::new(ctx.accounts.gauge_slot.key(), false),
                AccountMeta::new_readonly(ctx.accounts.student.key(), false),
            ],
            data: score.to_le_bytes().to_vec(),
        };
        invoke(&fixed_ix, &[
            ctx.accounts.gauge_marker.to_account_info(),
            ctx.accounts.gauge_slot.to_account_info(),
            ctx.accounts.student.to_account_info(),
        ])?;

        let mut prog = ctx.accounts.signal_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            prog = ctx.remaining_accounts[0].clone();
            ctx.accounts.journal.paths = ctx.accounts.journal.paths.saturating_add(5);
        }
        let dyn_ix = Instruction {
            program_id: *prog.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.signal_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.student.key(), false),
            ],
            data: gift.to_le_bytes().to_vec(),
        };
        invoke(&dyn_ix, &[
            prog,
            ctx.accounts.signal_board.to_account_info(),
            ctx.accounts.student.to_account_info(),
        ])?;

        let t = Transfer {
            from: ctx.accounts.reward_pool.to_account_info(),
            to: ctx.accounts.student_token.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info(),
        };
        let tctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), t);
        token::transfer(tctx, gift)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Submit<'info> {
    #[account(mut)]
    pub journal: Account<'info, GradeJournal>,
    /// CHECK:
    pub gauge_slot: AccountInfo<'info>,
    /// CHECK:
    pub student: AccountInfo<'info>,
    /// CHECK:
    pub gauge_marker: AccountInfo<'info>,
    /// CHECK:
    pub signal_board: AccountInfo<'info>,
    /// CHECK:
    pub signal_hint: AccountInfo<'info>,
    #[account(mut)]
    pub reward_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub student_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub pool_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct GradeJournal { pub stars: u64, pub warnings: u64, pub paths: u64 }
