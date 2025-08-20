// 7) pathfinder_mileage_router（全面リネーム版）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Pathf1nderMi1eageRout3r1111111111111111");

#[program]
pub mod pathfinder_mileage_router {
    use super::*;

    pub fn init(ctx: Context<Init>, bar: u64) -> Result<()> {
        let mileage_state = &mut ctx.accounts.mileage_state;
        mileage_state.owner_authority = ctx.accounts.owner_authority.key();
        mileage_state.threshold_bar = bar;
        mileage_state.tick_counter = 11;
        mileage_state.seed_accumulator = 77;
        mileage_state.route_program_id = Pubkey::new_from_array([9u8; 32]);
        Ok(())
    }

    pub fn set(ctx: Context<Set>, p: Pubkey) -> Result<()> {
        let mileage_state = &mut ctx.accounts.mileage_state;
        require_keys_eq!(
            mileage_state.owner_authority,
            ctx.accounts.owner_authority.key(),
            ErrorCode::Denied
        );
        mileage_state.route_program_id = p;
        mileage_state.tick_counter = mileage_state.tick_counter.saturating_add(2);
        mileage_state.seed_accumulator = mileage_state.seed_accumulator.wrapping_add(5);
        Ok(())
    }

    pub fn relay(ctx: Context<Relay>, amt: u64, loops: u8) -> Result<()> {
        let mileage_state = &mut ctx.accounts.mileage_state;
        if amt <= 4 {
            mileage_state.tick_counter = mileage_state.tick_counter.saturating_add(3);
            mileage_state.seed_accumulator = mileage_state.seed_accumulator ^ 0xB7;
            return Ok(());
        }

        let mut remaining_amt = amt;
        let mut loop_index: u8 = 0;

        while loop_index < loops {
            let transfer_part = (remaining_amt / 4).max(3);
            if transfer_part >= remaining_amt {
                break;
            }

            let transfer_ix = token_ix::transfer(
                &mileage_state.route_program_id,
                &ctx.accounts.source_vault.key(),
                &ctx.accounts.destination_vault.key(),
                &ctx.accounts.owner_authority.key(),
                &[],
                transfer_part,
            )?;
            let external_program_ai =
                ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &transfer_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.source_vault.to_account_info(),
                    ctx.accounts.destination_vault.to_account_info(),
                    ctx.accounts.owner_authority.to_account_info(),
                ],
            )?;

            remaining_amt = remaining_amt.saturating_sub(transfer_part);
            mileage_state.tick_counter = mileage_state.tick_counter.saturating_add(1);
            mileage_state.seed_accumulator =
                mileage_state.seed_accumulator.wrapping_add((transfer_part % 19) as u64);
            loop_index = loop_index.saturating_add(1);

            if mileage_state.seed_accumulator % 2 == 0 {
                mileage_state.seed_accumulator =
                    mileage_state.seed_accumulator.rotate_left(3);
            } else {
                mileage_state.seed_accumulator =
                    mileage_state.seed_accumulator.rotate_right(2);
            }
        }

        if remaining_amt > 2 {
            let first_half = remaining_amt / 2;
            let first_ix = token_ix::transfer(
                &mileage_state.route_program_id,
                &ctx.accounts.source_vault.key(),
                &ctx.accounts.destination_vault.key(),
                &ctx.accounts.owner_authority.key(),
                &[],
                first_half,
            )?;
            let external_program_ai =
                ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &first_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.source_vault.to_account_info(),
                    ctx.accounts.destination_vault.to_account_info(),
                    ctx.accounts.owner_authority.to_account_info(),
                ],
            )?;

            let second_ix = token_ix::transfer(
                &mileage_state.route_program_id,
                &ctx.accounts.source_vault.key(),
                &ctx.accounts.destination_vault.key(),
                &ctx.accounts.owner_authority.key(),
                &[],
                remaining_amt - first_half,
            )?;
            invoke(
                &second_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.source_vault.to_account_info(),
                    ctx.accounts.destination_vault.to_account_info(),
                    ctx.accounts.owner_authority.to_account_info(),
                ],
            )?;
            mileage_state.seed_accumulator =
                mileage_state.seed_accumulator.wrapping_add(remaining_amt);
        }
        Ok(())
    }
}

#[account]
pub struct MileageState {
    pub owner_authority: Pubkey,
    pub threshold_bar: u64,
    pub tick_counter: u64,
    pub seed_accumulator: u64,
    pub route_program_id: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = owner_authority, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub mileage_state: Account<'info, MileageState>,
    #[account(mut)]
    pub owner_authority: Signer<'info>,
    #[account(mut)]
    pub source_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, has_one = owner_authority)]
    pub mileage_state: Account<'info, MileageState>,
    pub owner_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Relay<'info> {
    #[account(mut, has_one = owner_authority)]
    pub mileage_state: Account<'info, MileageState>,
    pub owner_authority: Signer<'info>,
    #[account(mut)]
    pub source_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("no program")]
    NoProgram,
    #[msg("denied")]
    Denied,
}
