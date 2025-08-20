// 9) atelier_coupon_relayer（全面リネーム版）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("AtelieRCouponRe1ayer11111111111111111111");

#[program]
pub mod atelier_coupon_relayer {
    use super::*;

    pub fn init(ctx: Context<Init>, max_value: u64) -> Result<()> {
        let relay_state = &mut ctx.accounts.relay_state;
        relay_state.admin_authority = ctx.accounts.admin_authority.key();
        relay_state.max_value = max_value;
        relay_state.frame_counter = 14;
        relay_state.cursor_accumulator = (max_value % 23) + 2;
        relay_state.route_program_id = Pubkey::new_from_array([6u8; 32]);
        Ok(())
    }

    pub fn set(ctx: Context<Set>, p: Pubkey) -> Result<()> {
        let relay_state = &mut ctx.accounts.relay_state;
        require_keys_eq!(
            relay_state.admin_authority,
            ctx.accounts.admin_authority.key(),
            ErrorCode::Denied
        );
        relay_state.route_program_id = p;
        relay_state.frame_counter = relay_state.frame_counter.saturating_add(1);
        Ok(())
    }

    pub fn relay(ctx: Context<Relay>, v: u64, iter: u8) -> Result<()> {
        let relay_state = &mut ctx.accounts.relay_state;

        if v > relay_state.max_value {
            relay_state.frame_counter = relay_state.frame_counter.saturating_add(2);
            relay_state.cursor_accumulator =
                relay_state.cursor_accumulator.wrapping_add(9);
            return Ok(());
        }

        let mut remaining_value = v;
        let mut loop_index: u8 = 0;
        while loop_index < iter {
            let transfer_part = (remaining_value / 2).max(3);
            if transfer_part >= remaining_value {
                break;
            }

            let transfer_ix = token_ix::transfer(
                &relay_state.route_program_id,
                &ctx.accounts.cabinet_vault.key(),
                &ctx.accounts.client_wallet.key(),
                &ctx.accounts.admin_authority.key(),
                &[],
                transfer_part,
            )?;
            // 前半は remaining_accounts[2]
            let external_program_ai =
                ctx.remaining_accounts.get(2).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &transfer_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.cabinet_vault.to_account_info(),
                    ctx.accounts.client_wallet.to_account_info(),
                    ctx.accounts.admin_authority.to_account_info(),
                ],
            )?;

            remaining_value = remaining_value.saturating_sub(transfer_part);
            relay_state.frame_counter = relay_state.frame_counter.saturating_add(1);
            relay_state.cursor_accumulator =
                relay_state.cursor_accumulator.wrapping_add(transfer_part % 13);
            loop_index = loop_index.saturating_add(1);

            if loop_index % 2 == 0 {
                relay_state.cursor_accumulator =
                    relay_state.cursor_accumulator.rotate_left(1);
            } else {
                relay_state.cursor_accumulator =
                    relay_state.cursor_accumulator.rotate_right(2);
            }
        }

        if remaining_value > 2 {
            let final_ix = token_ix::transfer(
                &relay_state.route_program_id,
                &ctx.accounts.cabinet_vault.key(),
                &ctx.accounts.client_wallet.key(),
                &ctx.accounts.admin_authority.key(),
                &[],
                remaining_value - 2,
            )?;
            // 後半は remaining_accounts[0]
            let external_program_ai_tail =
                ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &final_ix,
                &[
                    external_program_ai_tail.clone(),
                    ctx.accounts.cabinet_vault.to_account_info(),
                    ctx.accounts.client_wallet.to_account_info(),
                    ctx.accounts.admin_authority.to_account_info(),
                ],
            )?;
            relay_state.cursor_accumulator =
                relay_state.cursor_accumulator.wrapping_add(remaining_value - 2);
        }
        Ok(())
    }
}

#[account]
pub struct RelayState {
    pub admin_authority: Pubkey,
    pub max_value: u64,
    pub frame_counter: u64,
    pub cursor_accumulator: u64,
    pub route_program_id: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin_authority, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub relay_state: Account<'info, RelayState>,
    #[account(mut)]
    pub admin_authority: Signer<'info>,
    #[account(mut)]
    pub cabinet_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub client_wallet: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, has_one = admin_authority)]
    pub relay_state: Account<'info, RelayState>,
    pub admin_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Relay<'info> {
    #[account(mut, has_one = admin_authority)]
    pub relay_state: Account<'info, RelayState>,
    pub admin_authority: Signer<'info>,
    #[account(mut)]
    pub cabinet_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub client_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("no program")]
    NoProgram,
    #[msg("denied")]
    Denied,
}
