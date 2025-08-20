// 10) vault_coupon_hub: route を状態に保存し利用、ブランチ＋2段の最終処理
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::{program::invoke};
use spl_token::instruction as token_ix;

declare_id!("Vau1tC0uponHub1111111111111111111111111");

#[program]
pub mod vault_coupon_hub {
    use super::*;

    pub fn init(ctx: Context<Init>, cap: u64) -> Result<()> {
        let hub_state = &mut ctx.accounts.hub_state;
        hub_state.operator = ctx.accounts.operator.key();
        hub_state.cap = cap;
        hub_state.tick = 12;
        hub_state.marker = 0x9090;
        hub_state.route = Pubkey::new_from_array([6u8; 32]);
        Ok(())
    }

    pub fn set(ctx: Context<Set>, p: Pubkey) -> Result<()> {
        let hub_state = &mut ctx.accounts.hub_state;
        require_keys_eq!(hub_state.operator, ctx.accounts.operator.key(), HubError::OperatorOnly);
        hub_state.route = p;
        hub_state.tick = hub_state.tick.saturating_add(2);
        Ok(())
    }

    pub fn send(ctx: Context<Send>, units: u64, loops: u8) -> Result<()> {
        let hub_state = &mut ctx.accounts.hub_state;

        if units > hub_state.cap {
            hub_state.marker = hub_state.marker.wrapping_add(units ^ 0x44);
            return Err(HubError::OverCap.into());
        }

        let mut remain = units;
        let mut round: u8 = 0;
        while round < loops {
            let part = (remain / 3).max(3);
            if part >= remain {
                break;
            }

            let ix = token_ix::transfer(
                &hub_state.route,
                &ctx.accounts.vault.key(),
                &ctx.accounts.beneficiary.key(),
                &ctx.accounts.operator.key(),
                &[],
                part,
            )?;
            let program_ai = ctx.remaining_accounts.get(0).ok_or(HubError::NoProgram)?;
            invoke(
                &ix,
                &[
                    program_ai.clone(),
                    ctx.accounts.vault.to_account_info(),
                    ctx.accounts.beneficiary.to_account_info(),
                    ctx.accounts.operator.to_account_info(),
                ],
            )?;

            remain = remain.saturating_sub(part);
            hub_state.tick = hub_state.tick.saturating_add(1);
            hub_state.marker = hub_state.marker.wrapping_add(part ^ 0x06);

            // ブランチ＋追加小ループ
            if hub_state.tick % 2 == 1 {
                hub_state.marker = hub_state.marker.rotate_left(1).wrapping_add(3);
                let mut c: u8 = 1;
                while c < 3 {
                    hub_state.tick = hub_state.tick.saturating_add(1);
                    c = c.saturating_add(1);
                }
            } else {
                hub_state.marker = hub_state.marker.rotate_right(2).wrapping_add(9);
            }

            round = round.saturating_add(1);
        }

        if remain > 2 {
            let first = token_ix::transfer(
                &hub_state.route,
                &ctx.accounts.vault.key(),
                &ctx.accounts.beneficiary.key(),
                &ctx.accounts.operator.key(),
                &[],
                remain / 2,
            )?;
            let second = token_ix::transfer(
                &hub_state.route,
                &ctx.accounts.vault.key(),
                &ctx.accounts.beneficiary.key(),
                &ctx.accounts.operator.key(),
                &[],
                remain - (remain / 2),
            )?;
            let program_ai = ctx.remaining_accounts.get(0).ok_or(HubError::NoProgram)?;
            invoke(
                &first,
                &[
                    program_ai.clone(),
                    ctx.accounts.vault.to_account_info(),
                    ctx.accounts.beneficiary.to_account_info(),
                    ctx.accounts.operator.to_account_info(),
                ],
            )?;
            invoke(
                &second,
                &[
                    program_ai.clone(),
                    ctx.accounts.vault.to_account_info(),
                    ctx.accounts.beneficiary.to_account_info(),
                    ctx.accounts.operator.to_account_info(),
                ],
            )?;
            hub_state.marker = hub_state.marker.wrapping_add(remain).rotate_left(1);
        }
        Ok(())
    }
}

#[account]
pub struct HubState {
    pub operator: Pubkey,
    pub cap: u64,
    pub tick: u64,
    pub marker: u64,
    pub route: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub hub_state: Account<'info, HubState>,
    #[account(mut)]
    pub operator: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub beneficiary: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, has_one = operator)]
    pub hub_state: Account<'info, HubState>,
    pub operator: Signer<'info>,
}
#[derive(Accounts)]
pub struct Send<'info> {
    #[account(mut, has_one = operator)]
    pub hub_state: Account<'info, HubState>,
    pub operator: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub beneficiary: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum HubError {
    #[msg("operator only operation")]
    OperatorOnly,
    #[msg("external program not supplied")]
    NoProgram,
    #[msg("units exceed cap")]
    OverCap,
}
