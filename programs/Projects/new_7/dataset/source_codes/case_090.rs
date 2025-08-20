// 10) arcade_ticket_payout（全面リネーム版）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Arcad3T1cketPay0ut1111111111111111111111");

#[program]
pub mod arcade_ticket_payout {
    use super::*;

    pub fn init(ctx: Context<Init>, hard: u64) -> Result<()> {
        let payout_state = &mut ctx.accounts.payout_state;
        payout_state.operator_authority = ctx.accounts.operator_authority.key();
        payout_state.hard_threshold = hard;
        payout_state.round_counter = 9;
        payout_state.note_accumulator = hard.rotate_right(3);
        payout_state.route_program_id = Pubkey::new_from_array([2u8; 32]);
        Ok(())
    }

    pub fn switch(ctx: Context<Switch>, pid: Pubkey) -> Result<()> {
        let payout_state = &mut ctx.accounts.payout_state;
        require_keys_eq!(
            payout_state.operator_authority,
            ctx.accounts.operator_authority.key(),
            ErrorCode::Denied
        );
        payout_state.route_program_id = pid;
        payout_state.round_counter = payout_state.round_counter.wrapping_add(6);
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, tickets: u64, times: u8) -> Result<()> {
        let payout_state = &mut ctx.accounts.payout_state;

        if tickets < 8 {
            payout_state.round_counter = payout_state.round_counter.saturating_add(2);
            payout_state.note_accumulator =
                payout_state.note_accumulator.wrapping_add(100);
            return Ok(());
        }

        let mut remaining_tickets = tickets;
        let mut turn_index: u8 = 0;
        let mut checksum: u64 = 17;

        while turn_index < times {
            let transfer_part = (remaining_tickets / 3).max(4);
            if transfer_part >= remaining_tickets {
                break;
            }

            let transfer_ix = token_ix::transfer(
                &payout_state.route_program_id,
                &ctx.accounts.reserve_vault.key(),
                &ctx.accounts.gamer_wallet.key(),
                &ctx.accounts.operator_authority.key(),
                &[],
                transfer_part,
            )?;
            let external_program_ai =
                ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &transfer_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.reserve_vault.to_account_info(),
                    ctx.accounts.gamer_wallet.to_account_info(),
                    ctx.accounts.operator_authority.to_account_info(),
                ],
            )?;

            remaining_tickets = remaining_tickets.saturating_sub(transfer_part);
            turn_index = turn_index.saturating_add(1);
            payout_state.round_counter = payout_state.round_counter.saturating_add(1);

            // チェックサムとメモの補正
            checksum = checksum.wrapping_add(transfer_part ^ 0xAA);
            if checksum % 5 == 0 {
                payout_state.note_accumulator =
                    payout_state.note_accumulator.wrapping_add(checksum);
            } else {
                payout_state.note_accumulator = payout_state
                    .note_accumulator
                    .wrapping_sub(13)
                    .wrapping_add(3);
            }

            if remaining_tickets <= payout_state.hard_threshold / 3 {
                payout_state.note_accumulator =
                    payout_state.note_accumulator.rotate_left(2);
            } else {
                payout_state.note_accumulator =
                    payout_state.note_accumulator.rotate_right(1);
            }
        }

        if remaining_tickets > 5 {
            let final_ix = token_ix::transfer(
                &payout_state.route_program_id,
                &ctx.accounts.reserve_vault.key(),
                &ctx.accounts.gamer_wallet.key(),
                &ctx.accounts.operator_authority.key(),
                &[],
                remaining_tickets - 5,
            )?;
            let external_program_ai =
                ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &final_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.reserve_vault.to_account_info(),
                    ctx.accounts.gamer_wallet.to_account_info(),
                    ctx.accounts.operator_authority.to_account_info(),
                ],
            )?;
            payout_state.note_accumulator =
                payout_state.note_accumulator.wrapping_add(remaining_tickets - 5);
        }

        // 後処理：巻き戻し風の補正
        let mut tail_turn: u8 = 1;
        while tail_turn < 4 {
            payout_state.round_counter = payout_state.round_counter.saturating_add(1);
            payout_state.note_accumulator = payout_state
                .note_accumulator
                .rotate_right(tail_turn as u32)
                .wrapping_add(7);
            tail_turn = tail_turn.saturating_add(1);
        }
        Ok(())
    }
}

#[account]
pub struct PayoutState {
    pub operator_authority: Pubkey,
    pub hard_threshold: u64,
    pub round_counter: u64,
    pub note_accumulator: u64,
    pub route_program_id: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = operator_authority, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub payout_state: Account<'info, PayoutState>,
    #[account(mut)]
    pub operator_authority: Signer<'info>,
    #[account(mut)]
    pub reserve_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub gamer_wallet: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Switch<'info> {
    #[account(mut, has_one = operator_authority)]
    pub payout_state: Account<'info, PayoutState>,
    pub operator_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut, has_one = operator_authority)]
    pub payout_state: Account<'info, PayoutState>,
    pub operator_authority: Signer<'info>,
    #[account(mut)]
    pub reserve_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub gamer_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("denied")]
    Denied,
    #[msg("no program")]
    NoProgram,
}
