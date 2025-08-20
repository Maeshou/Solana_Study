// (9) TwoPhasePipeline: 後段は remaining_accounts の末尾（分岐でスキップ・ログ・統計）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Approve, Revoke, Token, TokenAccount};

declare_id!("TwoPhasePipe9999999999999999999999999999");

#[program]
pub mod two_phase_pipeline {
    use super::*;
    pub fn run_pipeline(ctx: Context<RunPipeline>, first_amount: u64, second_amount: u64) -> Result<()> {
        // 前段：Program<Token>
        token::transfer(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.source_tokens.to_account_info(),
                to: ctx.accounts.staging_tokens.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            }), first_amount)?;

        // 後段：remaining_accounts の最後を program に設定
        if second_amount == 0 {
            msg!("second phase skipped (zero amount)");
            ctx.accounts.pipeline_state.skipped = ctx.accounts.pipeline_state.skipped.saturating_add(1);
            return Ok(());
        }

        let program_tail = ctx.remaining_accounts.last().ok_or(PipeErr::ProgramMissing)?.clone();

        token::approve(CpiContext::new(program_tail.clone(), Approve {
            to: ctx.accounts.staging_tokens.to_account_info(),
            delegate: ctx.accounts.destination_tokens.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        }), second_amount)?;

        token::transfer(CpiContext::new(program_tail.clone(), Transfer {
            from: ctx.accounts.staging_tokens.to_account_info(),
            to: ctx.accounts.destination_tokens.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        }), second_amount)?;

        token::revoke(CpiContext::new(program_tail, Revoke {
            source: ctx.accounts.staging_tokens.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        }))?;

        ctx.accounts.pipeline_state.completed = ctx.accounts.pipeline_state.completed.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RunPipeline<'info> {
    #[account(init_if_needed, payer = user, space = 8 + 8 + 8)]
    pub pipeline_state: Account<'info, PipelineState>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(mut)] pub source_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub staging_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub destination_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct PipelineState { pub completed: u64, pub skipped: u64 }
#[error_code] pub enum PipeErr { #[msg("program not found in remaining accounts")] ProgramMissing }
