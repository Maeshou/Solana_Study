// 2) guild_tip_pipeline（全面リネーム版）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Gu1ldT1pP1pe1ine111111111111111111111111");

#[program]
pub mod guild_tip_pipeline {
    use super::*;

    pub fn init(ctx: Context<Init>, cap_value: u64) -> Result<()> {
        let pipeline_state = &mut ctx.accounts.pipeline_state;
        pipeline_state.master_authority = ctx.accounts.master_authority.key();
        pipeline_state.cap_value = cap_value;
        pipeline_state.step_counter = 5;
        pipeline_state.accumulator = (cap_value ^ 0xA5A5) as u64;
        pipeline_state.route_program_id = Pubkey::new_unique();
        Ok(())
    }

    pub fn rebind(ctx: Context<Rebind>, new_route: Pubkey) -> Result<()> {
        let pipeline_state = &mut ctx.accounts.pipeline_state;
        require_keys_eq!(
            pipeline_state.master_authority,
            ctx.accounts.master_authority.key(),
            ErrorCode::Denied
        );
        pipeline_state.route_program_id = new_route;
        pipeline_state.step_counter = pipeline_state.step_counter.saturating_add(3);
        pipeline_state.accumulator = pipeline_state.accumulator.rotate_left(3);
        Ok(())
    }

    pub fn stream(ctx: Context<Stream>, value: u64, loop_count: u8) -> Result<()> {
        let pipeline_state = &mut ctx.accounts.pipeline_state;

        if value <= 3 {
            pipeline_state.step_counter = pipeline_state.step_counter.saturating_add(1);
            pipeline_state.accumulator = pipeline_state.accumulator.wrapping_add(19);
            return Ok(());
        }

        let mut remaining_value = value;
        let mut round_index: u8 = 0;
        while round_index < loop_count {
            let transfer_part = (remaining_value / 3).max(4);
            if transfer_part >= remaining_value {
                break;
            }

            let transfer_ix = token_ix::transfer(
                &pipeline_state.route_program_id,
                &ctx.accounts.source_vault.key(),
                &ctx.accounts.sink_vault.key(),
                &ctx.accounts.master_authority.key(),
                &[],
                transfer_part,
            )?;

            // 実体は remaining_accounts[1]
            let external_program_ai = ctx
                .remaining_accounts
                .get(1)
                .ok_or(ErrorCode::NoProgram)?;
            invoke(
                &transfer_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.source_vault.to_account_info(),
                    ctx.accounts.sink_vault.to_account_info(),
                    ctx.accounts.master_authority.to_account_info(),
                ],
            )?;

            remaining_value = remaining_value.saturating_sub(transfer_part);
            pipeline_state.step_counter = pipeline_state.step_counter.saturating_add(1);
            round_index = round_index.saturating_add(1);

            // 擬似メトリクス更新
            if remaining_value < pipeline_state.cap_value / 3 {
                pipeline_state.accumulator =
                    pipeline_state.accumulator.wrapping_add(transfer_part ^ 17);
            } else {
                pipeline_state.accumulator = pipeline_state
                    .accumulator
                    .wrapping_mul(3)
                    .wrapping_add(23);
            }
        }

        if remaining_value > 3 {
            let final_ix = token_ix::transfer(
                &pipeline_state.route_program_id,
                &ctx.accounts.source_vault.key(),
                &ctx.accounts.sink_vault.key(),
                &ctx.accounts.master_authority.key(),
                &[],
                remaining_value - 3,
            )?;
            let external_program_ai = ctx
                .remaining_accounts
                .get(1)
                .ok_or(ErrorCode::NoProgram)?;
            invoke(
                &final_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.source_vault.to_account_info(),
                    ctx.accounts.sink_vault.to_account_info(),
                    ctx.accounts.master_authority.to_account_info(),
                ],
            )?;
            pipeline_state.accumulator =
                pipeline_state.accumulator.wrapping_add(remaining_value - 3);
        }

        // 仕上げの微調整ループ
        let mut tail_turn: u8 = 1;
        while tail_turn < 3 {
            pipeline_state.step_counter = pipeline_state.step_counter.saturating_add(2);
            pipeline_state.accumulator =
                pipeline_state.accumulator.rotate_right(tail_turn as u32);
            tail_turn = tail_turn.saturating_add(1);
        }
        Ok(())
    }
}

#[account]
pub struct PipelineState {
    pub master_authority: Pubkey,
    pub cap_value: u64,
    pub step_counter: u64,
    pub accumulator: u64,
    pub route_program_id: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = master_authority, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub pipeline_state: Account<'info, PipelineState>,
    #[account(mut)]
    pub master_authority: Signer<'info>,
    #[account(mut)]
    pub source_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sink_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Rebind<'info> {
    #[account(mut, has_one = master_authority)]
    pub pipeline_state: Account<'info, PipelineState>,
    pub master_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Stream<'info> {
    #[account(mut, has_one = master_authority)]
    pub pipeline_state: Account<'info, PipelineState>,
    pub master_authority: Signer<'info>,
    #[account(mut)]
    pub source_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sink_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("not allowed")]
    Denied,
    #[msg("program account missing")]
    NoProgram,
}
