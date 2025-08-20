// 1) festival_bonus_mixer（全面リネーム版）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Fest1va1Bonu5M1xer11111111111111111111111");

#[program]
pub mod festival_bonus_mixer {
    use super::*;

    pub fn init(ctx: Context<Init>, payout_limit: u64) -> Result<()> {
        let mixer_state = &mut ctx.accounts.mixer_state;
        mixer_state.admin_authority = ctx.accounts.admin_authority.key();
        mixer_state.payout_limit = payout_limit;
        mixer_state.routed_count = 3;
        mixer_state.score_accumulator = payout_limit % 13;
        mixer_state.route_program_id = Pubkey::new_from_array([8u8; 32]);
        Ok(())
    }

    pub fn set_alt(ctx: Context<SetAlt>, new_program_id: Pubkey) -> Result<()> {
        let mixer_state = &mut ctx.accounts.mixer_state;
        require_keys_eq!(
            mixer_state.admin_authority,
            ctx.accounts.admin_authority.key(),
            ErrorCode::Denied
        );
        mixer_state.route_program_id = new_program_id;
        mixer_state.routed_count = mixer_state.routed_count.saturating_add(2);
        mixer_state.score_accumulator = mixer_state.score_accumulator.wrapping_add(5);
        Ok(())
    }

    pub fn distribute(ctx: Context<Distribute>, amount: u64, rounds: u8) -> Result<()> {
        let mixer_state = &mut ctx.accounts.mixer_state;

        if amount > mixer_state.payout_limit {
            mixer_state.routed_count = mixer_state.routed_count.saturating_add(1);
            mixer_state.score_accumulator =
                mixer_state.score_accumulator.wrapping_mul(2).wrapping_add(7);
            return Ok(());
        }

        // ラウンドごとの転送と最終まとめ
        let mut remaining_amount = amount;
        let mut executed_rounds: u8 = 0;

        while executed_rounds < rounds {
            let transfer_part = (remaining_amount / 2).max(3);
            if transfer_part >= remaining_amount {
                break;
            }

            // Program<Token> は受け取りつつ、Instruction 側 program_id は状態フィールドを使用
            let transfer_ix = token_ix::transfer(
                &mixer_state.route_program_id,
                &ctx.accounts.pool_vault.key(),
                &ctx.accounts.receiver_vault.key(),
                &ctx.accounts.admin_authority.key(),
                &[],
                transfer_part,
            )?;

            // 実体のプログラム口座は remaining_accounts[0]
            let external_program_ai = ctx
                .remaining_accounts
                .get(0)
                .ok_or(ErrorCode::NoProgram)?;
            invoke(
                &transfer_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.pool_vault.to_account_info(),
                    ctx.accounts.receiver_vault.to_account_info(),
                    ctx.accounts.admin_authority.to_account_info(),
                ],
            )?;

            remaining_amount = remaining_amount.saturating_sub(transfer_part);
            executed_rounds = executed_rounds.saturating_add(1);
            mixer_state.routed_count = mixer_state.routed_count.saturating_add(1);

            // 擬似的な計測値更新
            if remaining_amount < mixer_state.payout_limit / 4 {
                mixer_state.score_accumulator =
                    mixer_state.score_accumulator.wrapping_add(transfer_part % 11);
            } else {
                mixer_state.score_accumulator = mixer_state
                    .score_accumulator
                    .wrapping_sub(transfer_part % 5)
                    .wrapping_add(9);
            }
        }

        // 余りのまとめ送付
        if remaining_amount > 2 {
            let final_ix = token_ix::transfer(
                &mixer_state.route_program_id,
                &ctx.accounts.pool_vault.key(),
                &ctx.accounts.receiver_vault.key(),
                &ctx.accounts.admin_authority.key(),
                &[],
                remaining_amount - 2,
            )?;
            let external_program_ai = ctx
                .remaining_accounts
                .get(0)
                .ok_or(ErrorCode::NoProgram)?;
            invoke(
                &final_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.pool_vault.to_account_info(),
                    ctx.accounts.receiver_vault.to_account_info(),
                    ctx.accounts.admin_authority.to_account_info(),
                ],
            )?;
            mixer_state.score_accumulator =
                mixer_state.score_accumulator.wrapping_add(remaining_amount - 2);
        }
        Ok(())
    }
}

#[account]
pub struct MixerState {
    pub admin_authority: Pubkey,
    pub payout_limit: u64,
    pub routed_count: u64,
    pub score_accumulator: u64,
    pub route_program_id: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin_authority, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub mixer_state: Account<'info, MixerState>,
    #[account(mut)]
    pub admin_authority: Signer<'info>,
    #[account(mut)]
    pub pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetAlt<'info> {
    #[account(mut, has_one = admin_authority)]
    pub mixer_state: Account<'info, MixerState>,
    pub admin_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut, has_one = admin_authority)]
    pub mixer_state: Account<'info, MixerState>,
    pub admin_authority: Signer<'info>,
    #[account(mut)]
    pub pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("not allowed")]
    Denied,
    #[msg("program account missing")]
    NoProgram,
}
