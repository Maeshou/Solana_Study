// (9) TwoPhasePipeline: 前段は固定、後段は remaining_accounts の末尾を使用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

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
        let program_tail = ctx.remaining_accounts.last().ok_or(PipeErr::ProgramMissing)?.clone();
        token::transfer(CpiContext::new(
            program_tail,
            Transfer {
                from: ctx.accounts.staging_tokens.to_account_info(),
                to: ctx.accounts.destination_tokens.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            }), second_amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RunPipeline<'info> {
    pub user: Signer<'info>,
    #[account(mut)] pub source_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub staging_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub destination_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code] pub enum PipeErr { #[msg("program not found in remaining accounts")] ProgramMissing }
