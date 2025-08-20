// 6) treasure_map_rebate: 2段フェーズ（前半と後半で調整），複数ブランチに追加処理
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::{program::invoke};
use spl_token::instruction as token_ix;

declare_id!("TreasureMapReb4te1111111111111111111111");

#[program]
pub mod treasure_map_rebate {
    use super::*;

    pub fn init(ctx: Context<Init>, ceiling: u64) -> Result<()> {
        let rebate_state = &mut ctx.accounts.rebate_state;
        rebate_state.captain = ctx.accounts.captain.key();
        rebate_state.ceiling = ceiling;
        rebate_state.iter = 10;
        rebate_state.radar = 0x4242;
        rebate_state.route_id = Pubkey::new_from_array([4u8; 32]);
        Ok(())
    }

    pub fn set_route(ctx: Context<SetRoute>, pid: Pubkey) -> Result<()> {
        let rebate_state = &mut ctx.accounts.rebate_state;
        require_keys_eq!(
            rebate_state.captain,
            ctx.accounts.captain.key(),
            RebateError::CaptainOnly
        );
        rebate_state.route_id = pid;
        rebate_state.iter = rebate_state.iter.saturating_add(2);
        Ok(())
    }

    pub fn rebate(ctx: Context<Rebate>, base: u64, rounds: u8) -> Result<()> {
        let rebate_state = &mut ctx.accounts.rebate_state;

        if base > rebate_state.ceiling {
            rebate_state.radar = rebate_state.radar.wrapping_add(base ^ 0x3C);
            return Err(RebateError::TooMuch.into());
        }

        let mut remaining = base;
        let mut r: u8 = 0;
        while r < rounds {
            let part = (remaining / 4).max(2);
            if part >= remaining {
                break;
            }
            let ix = token_ix::transfer(
                &rebate_state.route_id,
                &ctx.accounts.engine_vault.key(),
                &ctx.accounts.client_vault.key(),
                &ctx.accounts.captain.key(),
                &[],
                part,
            )?;
            let program_ai = ctx.remaining_accounts.get(0).ok_or(RebateError::NoProg)?;
            invoke(
                &ix,
                &[
                    program_ai.clone(),
                    ctx.accounts.engine_vault.to_account_info(),
                    ctx.accounts.client_vault.to_account_info(),
                    ctx.accounts.captain.to_account_info(),
                ],
            )?;

            remaining = remaining.saturating_sub(part);
            rebate_state.iter = rebate_state.iter.saturating_add(1);
            rebate_state.radar = rebate_state.radar.wrapping_add(part ^ 0x0C);

            // フェーズ内の補正
            if rebate_state.iter % 2 == 0 {
                let mut step: u8 = 1;
                while step < 3 {
                    rebate_state.radar =
                        rebate_state.radar.rotate_left((step % 2) as u32);
                    step = step.saturating_add(1);
                }
            } else {
                rebate_state.radar = rebate_state.radar.wrapping_add(9);
            }

            r = r.saturating_add(1);
        }

        if remaining > 3 {
            let ix2 = token_ix::transfer(
                &rebate_state.route_id,
                &ctx.accounts.engine_vault.key(),
                &ctx.accounts.client_vault.key(),
                &ctx.accounts.captain.key(),
                &[],
                remaining - 3,
            )?;
            let program_ai = ctx.remaining_accounts.get(0).ok_or(RebateError::NoProg)?;
            invoke(
                &ix2,
                &[
                    program_ai.clone(),
                    ctx.accounts.engine_vault.to_account_info(),
                    ctx.accounts.client_vault.to_account_info(),
                    ctx.accounts.captain.to_account_info(),
                ],
            )?;
            // 後処理ループ
            rebate_state.radar =
                rebate_state.radar.wrapping_add(remaining - 3).rotate_right(1);
            let mut clean: u8 = 1;
            while clean < 4 {
                rebate_state.iter = rebate_state.iter.saturating_add(1);
                clean = clean.saturating_add(1);
            }
        }
        Ok(())
    }
}

#[account]
pub struct RebateState {
    pub captain: Pubkey,
    pub ceiling: u64,
    pub iter: u64,
    pub radar: u64,
    pub route_id: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = captain, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub rebate_state: Account<'info, RebateState>,
    #[account(mut)]
    pub captain: Signer<'info>,
    #[account(mut)]
    pub engine_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub client_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SetRoute<'info> {
    #[account(mut, has_one = captain)]
    pub rebate_state: Account<'info, RebateState>,
    pub captain: Signer<'info>,
}
#[derive(Accounts)]
pub struct Rebate<'info> {
    #[account(mut, has_one = captain)]
    pub rebate_state: Account<'info, RebateState>,
    pub captain: Signer<'info>,
    #[account(mut)]
    pub engine_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub client_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum RebateError {
    #[msg("captain only operation")]
    CaptainOnly,
    #[msg("external program not provided")]
    NoProg,
    #[msg("requested amount exceeds ceiling")]
    TooMuch,
}
