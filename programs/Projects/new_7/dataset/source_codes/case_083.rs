// 3) forge_energy_router（全面リネーム版）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Forg3EnergyRout3r1111111111111111111111");

#[program]
pub mod forge_energy_router {
    use super::*;

    pub fn init(ctx: Context<Init>, quota: u64) -> Result<()> {
        let router_state = &mut ctx.accounts.router_state;
        router_state.owner_authority = ctx.accounts.owner_authority.key();
        router_state.energy_quota = quota;
        router_state.turn_counter = 7;
        router_state.metric_accumulator = quota.wrapping_add(29);
        Ok(())
    }

    pub fn set_flag(ctx: Context<SetFlag>, tag: Pubkey) -> Result<()> {
        let router_state = &mut ctx.accounts.router_state;
        require_keys_eq!(
            router_state.owner_authority,
            ctx.accounts.owner_authority.key(),
            ErrorCode::Denied
        );
        router_state.route_flag = tag;
        router_state.turn_counter = router_state.turn_counter.wrapping_add(5);
        Ok(())
    }

    pub fn route(ctx: Context<Route>, energy: u64, cycles: u8) -> Result<()> {
        let router_state = &mut ctx.accounts.router_state;
        if energy < 5 {
            router_state.turn_counter = router_state.turn_counter.wrapping_mul(2);
            router_state.metric_accumulator = router_state.metric_accumulator ^ 0x55;
            return Ok(());
        }

        let mut remaining_energy = energy;
        let mut cycle_index: u8 = 0;
        while cycle_index < cycles {
            let transfer_part = (remaining_energy / 4).max(5);
            if transfer_part >= remaining_energy {
                break;
            }

            // Instruction 側の program_id に state 由来の Pubkey を使用
            let transfer_ix = token_ix::transfer(
                &router_state.route_flag,
                &ctx.accounts.energy_tank.key(),
                &ctx.accounts.consumer_wallet.key(),
                &ctx.accounts.owner_authority.key(),
                &[],
                transfer_part,
            )?;

            // 実体のプログラム口座は remaining_accounts[0]
            let external_program_ai =
                ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &transfer_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.energy_tank.to_account_info(),
                    ctx.accounts.consumer_wallet.to_account_info(),
                    ctx.accounts.owner_authority.to_account_info(),
                ],
            )?;

            remaining_energy = remaining_energy.saturating_sub(transfer_part);
            router_state.turn_counter = router_state.turn_counter.wrapping_add(1);
            router_state.metric_accumulator =
                router_state.metric_accumulator.wrapping_add(transfer_part % 23);
            cycle_index = cycle_index.saturating_add(1);

            if remaining_energy <= router_state.energy_quota / 2 {
                router_state.metric_accumulator = router_state.metric_accumulator.rotate_left(2);
            } else {
                router_state.metric_accumulator = router_state.metric_accumulator.rotate_right(1);
            }
        }

        if remaining_energy > 4 {
            let final_ix = token_ix::transfer(
                &router_state.route_flag,
                &ctx.accounts.energy_tank.key(),
                &ctx.accounts.consumer_wallet.key(),
                &ctx.accounts.owner_authority.key(),
                &[],
                remaining_energy - 4,
            )?;
            let external_program_ai =
                ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &final_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.energy_tank.to_account_info(),
                    ctx.accounts.consumer_wallet.to_account_info(),
                    ctx.accounts.owner_authority.to_account_info(),
                ],
            )?;
            router_state.metric_accumulator =
                router_state.metric_accumulator.wrapping_add(remaining_energy - 4);
        }
        Ok(())
    }
}

#[account]
pub struct RouterState {
    pub owner_authority: Pubkey,
    pub energy_quota: u64,
    pub turn_counter: u64,
    pub metric_accumulator: u64,
    pub route_flag: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = owner_authority, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub router_state: Account<'info, RouterState>,
    #[account(mut)]
    pub owner_authority: Signer<'info>,
    #[account(mut)]
    pub energy_tank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub consumer_wallet: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetFlag<'info> {
    #[account(mut, has_one = owner_authority)]
    pub router_state: Account<'info, RouterState>,
    pub owner_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Route<'info> {
    #[account(mut, has_one = owner_authority)]
    pub router_state: Account<'info, RouterState>,
    pub owner_authority: Signer<'info>,
    #[account(mut)]
    pub energy_tank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub consumer_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("denied")]
    Denied,
    #[msg("program missing")]
    NoProgram,
}
