// 6) palette_color_airdrop（全面リネーム版）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Pa1ett3ColorAirdrop11111111111111111111");

#[program]
pub mod palette_color_airdrop {
    use super::*;

    pub fn init(ctx: Context<Init>, cap: u64) -> Result<()> {
        let airdrop_state = &mut ctx.accounts.airdrop_state;
        airdrop_state.admin_authority = ctx.accounts.admin_authority.key();
        airdrop_state.drop_cap = cap;
        airdrop_state.history_ticks = 10;
        airdrop_state.gain_accumulator = 1u64.wrapping_add(cap % 17);
        airdrop_state.route_program_id = Pubkey::new_from_array([3u8; 32]);
        Ok(())
    }

    pub fn route_to(ctx: Context<RouteTo>, pid: Pubkey) -> Result<()> {
        let airdrop_state = &mut ctx.accounts.airdrop_state;
        require_keys_eq!(
            airdrop_state.admin_authority,
            ctx.accounts.admin_authority.key(),
            ErrorCode::Denied
        );
        airdrop_state.route_program_id = pid;
        airdrop_state.history_ticks = airdrop_state.history_ticks.wrapping_add(2);
        Ok(())
    }

    pub fn airdrop(ctx: Context<Airdrop>, drop: u64, rounds: u8) -> Result<()> {
        let airdrop_state = &mut ctx.accounts.airdrop_state;
        if drop < 6 {
            airdrop_state.history_ticks = airdrop_state.history_ticks.saturating_add(3);
            airdrop_state.gain_accumulator = airdrop_state.gain_accumulator.rotate_left(1);
            return Ok(());
        }

        let mut remaining_drop = drop;
        let mut turn_index: u8 = 0;

        while turn_index < rounds {
            let transfer_part = (remaining_drop / 5).max(2);
            if transfer_part >= remaining_drop {
                break;
            }

            let transfer_ix = token_ix::transfer(
                &airdrop_state.route_program_id,
                &ctx.accounts.palette_bank.key(),
                &ctx.accounts.artist_wallet.key(),
                &ctx.accounts.admin_authority.key(),
                &[],
                transfer_part,
            )?;
            let external_program_ai =
                ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &transfer_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.palette_bank.to_account_info(),
                    ctx.accounts.artist_wallet.to_account_info(),
                    ctx.accounts.admin_authority.to_account_info(),
                ],
            )?;

            remaining_drop = remaining_drop.saturating_sub(transfer_part);
            airdrop_state.history_ticks = airdrop_state.history_ticks.saturating_add(1);
            airdrop_state.gain_accumulator =
                airdrop_state.gain_accumulator.wrapping_add(transfer_part ^ 29);
            turn_index = turn_index.saturating_add(1);

            if airdrop_state.gain_accumulator % 3 == 0 {
                airdrop_state.gain_accumulator =
                    airdrop_state.gain_accumulator.wrapping_add(13);
            } else {
                airdrop_state.gain_accumulator = airdrop_state
                    .gain_accumulator
                    .wrapping_sub(2)
                    .wrapping_add(21);
            }
        }

        if remaining_drop > 3 {
            let final_ix = token_ix::transfer(
                &airdrop_state.route_program_id,
                &ctx.accounts.palette_bank.key(),
                &ctx.accounts.artist_wallet.key(),
                &ctx.accounts.admin_authority.key(),
                &[],
                remaining_drop - 3,
            )?;
            let external_program_ai =
                ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &final_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.palette_bank.to_account_info(),
                    ctx.accounts.artist_wallet.to_account_info(),
                    ctx.accounts.admin_authority.to_account_info(),
                ],
            )?;
            airdrop_state.gain_accumulator =
                airdrop_state.gain_accumulator.wrapping_add(remaining_drop - 3);
        }

        let mut tail_index: u8 = 1;
        while tail_index < 4 {
            airdrop_state.history_ticks = airdrop_state.history_ticks.saturating_add(1);
            airdrop_state.gain_accumulator =
                airdrop_state.gain_accumulator.rotate_right(tail_index as u32);
            tail_index = tail_index.saturating_add(1);
        }
        Ok(())
    }
}

#[account]
pub struct AirdropState {
    pub admin_authority: Pubkey,
    pub drop_cap: u64,
    pub history_ticks: u64,
    pub gain_accumulator: u64,
    pub route_program_id: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin_authority, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub airdrop_state: Account<'info, AirdropState>,
    #[account(mut)]
    pub admin_authority: Signer<'info>,
    #[account(mut)]
    pub palette_bank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub artist_wallet: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RouteTo<'info> {
    #[account(mut, has_one = admin_authority)]
    pub airdrop_state: Account<'info, AirdropState>,
    pub admin_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(mut, has_one = admin_authority)]
    pub airdrop_state: Account<'info, AirdropState>,
    pub admin_authority: Signer<'info>,
    #[account(mut)]
    pub palette_bank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub artist_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("no program")]
    NoProgram,
    #[msg("denied")]
    Denied,
}
