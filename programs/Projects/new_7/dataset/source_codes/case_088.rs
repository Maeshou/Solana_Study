// 8) caravan_loyalty_engine（全面リネーム版）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("CaravanL0ya1tyEng1ne11111111111111111111");

#[program]
pub mod caravan_loyalty_engine {
    use super::*;

    pub fn init(ctx: Context<Init>, bar: u64) -> Result<()> {
        let loyalty_state = &mut ctx.accounts.loyalty_state;
        loyalty_state.admin_authority = ctx.accounts.admin_authority.key();
        loyalty_state.threshold_bar = bar;
        loyalty_state.count_steps = 12;
        loyalty_state.hash_accumulator = bar.rotate_left(2);
        loyalty_state.alt_program_id = Pubkey::new_from_array([4u8; 32]);
        Ok(())
    }

    pub fn bind(ctx: Context<Bind>, id: Pubkey) -> Result<()> {
        let loyalty_state = &mut ctx.accounts.loyalty_state;
        require_keys_eq!(
            loyalty_state.admin_authority,
            ctx.accounts.admin_authority.key(),
            ErrorCode::Denied
        );
        loyalty_state.alt_program_id = id;
        loyalty_state.count_steps = loyalty_state.count_steps.wrapping_add(2);
        Ok(())
    }

    pub fn process(ctx: Context<Process>, n: u64, r: u8) -> Result<()> {
        let loyalty_state = &mut ctx.accounts.loyalty_state;
        if n < 3 {
            loyalty_state.hash_accumulator = loyalty_state.hash_accumulator ^ 0xFA;
            loyalty_state.count_steps = loyalty_state.count_steps.saturating_add(1);
            return Ok(());
        }

        let mut remaining_units = n;
        let mut round_index: u8 = 0;
        while round_index < r {
            let transfer_part = (remaining_units / 3).max(2);
            if transfer_part >= remaining_units {
                break;
            }

            let transfer_ix = token_ix::transfer(
                &loyalty_state.alt_program_id,
                &ctx.accounts.vault.key(),
                &ctx.accounts.user_wallet.key(),
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
                    ctx.accounts.vault.to_account_info(),
                    ctx.accounts.user_wallet.to_account_info(),
                    ctx.accounts.admin_authority.to_account_info(),
                ],
            )?;

            remaining_units = remaining_units.saturating_sub(transfer_part);
            loyalty_state.count_steps = loyalty_state.count_steps.saturating_add(1);
            loyalty_state.hash_accumulator = loyalty_state
                .hash_accumulator
                .wrapping_add(transfer_part)
                .rotate_right(2);
            round_index = round_index.saturating_add(1);

            if loyalty_state.count_steps % 3 == 1 {
                loyalty_state.hash_accumulator =
                    loyalty_state.hash_accumulator.wrapping_add(15);
            } else {
                loyalty_state.hash_accumulator = loyalty_state
                    .hash_accumulator
                    .wrapping_sub(6)
                    .wrapping_add(2);
            }
        }

        if remaining_units > 2 {
            let half = remaining_units / 2;
            let ix_first = token_ix::transfer(
                &loyalty_state.alt_program_id,
                &ctx.accounts.vault.key(),
                &ctx.accounts.user_wallet.key(),
                &ctx.accounts.admin_authority.key(),
                &[],
                half,
            )?;
            let external_program_ai =
                ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix_first,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.vault.to_account_info(),
                    ctx.accounts.user_wallet.to_account_info(),
                    ctx.accounts.admin_authority.to_account_info(),
                ],
            )?;
            let ix_second = token_ix::transfer(
                &loyalty_state.alt_program_id,
                &ctx.accounts.vault.key(),
                &ctx.accounts.user_wallet.key(),
                &ctx.accounts.admin_authority.key(),
                &[],
                remaining_units - half,
            )?;
            invoke(
                &ix_second,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.vault.to_account_info(),
                    ctx.accounts.user_wallet.to_account_info(),
                    ctx.accounts.admin_authority.to_account_info(),
                ],
            )?;
            loyalty_state.hash_accumulator =
                loyalty_state.hash_accumulator.wrapping_add(remaining_units);
        }
        Ok(())
    }
}

#[account]
pub struct LoyaltyState {
    pub admin_authority: Pubkey,
    pub threshold_bar: u64,
    pub count_steps: u64,
    pub hash_accumulator: u64,
    pub alt_program_id: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin_authority, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub loyalty_state: Account<'info, LoyaltyState>,
    #[account(mut)]
    pub admin_authority: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_wallet: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Bind<'info> {
    #[account(mut, has_one = admin_authority)]
    pub loyalty_state: Account<'info, LoyaltyState>,
    pub admin_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Process<'info> {
    #[account(mut, has_one = admin_authority)]
    pub loyalty_state: Account<'info, LoyaltyState>,
    pub admin_authority: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("no program")]
    NoProgram,
    #[msg("denied")]
    Denied,
}
