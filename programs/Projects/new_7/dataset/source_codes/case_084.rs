// 4) market_cashback_splitter（全面リネーム版）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Marke7CashbackSp1it111111111111111111111");

#[program]
pub mod market_cashback_splitter {
    use super::*;

    pub fn init(ctx: Context<Init>, ceiling: u64) -> Result<()> {
        let splitter_state = &mut ctx.accounts.splitter_state;
        splitter_state.chair_authority = ctx.accounts.chair_authority.key();
        splitter_state.cashback_ceiling = ceiling;
        splitter_state.round_counter = 6;
        splitter_state.audit_accumulator = ceiling.wrapping_mul(3);
        splitter_state.route_plug = Pubkey::new_from_array([5u8; 32]);
        Ok(())
    }

    pub fn plug(ctx: Context<Plug>, id: Pubkey) -> Result<()> {
        let splitter_state = &mut ctx.accounts.splitter_state;
        require_keys_eq!(
            splitter_state.chair_authority,
            ctx.accounts.chair_authority.key(),
            ErrorCode::Denied
        );
        splitter_state.route_plug = id;
        splitter_state.round_counter = splitter_state.round_counter.wrapping_add(4);
        Ok(())
    }

    pub fn run(ctx: Context<Run>, cash: u64, rounds: u8) -> Result<()> {
        let splitter_state = &mut ctx.accounts.splitter_state;
        if cash >= splitter_state.cashback_ceiling {
            splitter_state.round_counter = splitter_state.round_counter.saturating_add(2);
            splitter_state.audit_accumulator = splitter_state.audit_accumulator ^ 0x33;
            return Ok(());
        }

        let mut remaining_cash = cash;
        let mut round_index: u8 = 0;
        while round_index < rounds {
            let transfer_part = (remaining_cash / 3).max(2);
            if transfer_part >= remaining_cash {
                break;
            }

            let transfer_ix = token_ix::transfer(
                &splitter_state.route_plug,
                &ctx.accounts.market_treasury.key(),
                &ctx.accounts.customer_wallet.key(),
                &ctx.accounts.chair_authority.key(),
                &[],
                transfer_part,
            )?;

            // Program を AccountInfo で受け取って invoke
            invoke(
                &transfer_ix,
                &[
                    ctx.accounts.external_program.clone(),
                    ctx.accounts.market_treasury.to_account_info(),
                    ctx.accounts.customer_wallet.to_account_info(),
                    ctx.accounts.chair_authority.to_account_info(),
                ],
            )?;

            remaining_cash = remaining_cash.saturating_sub(transfer_part);
            splitter_state.round_counter = splitter_state.round_counter.saturating_add(1);
            splitter_state.audit_accumulator = splitter_state
                .audit_accumulator
                .wrapping_add(transfer_part)
                .rotate_left(1);
            round_index = round_index.saturating_add(1);

            if splitter_state.round_counter % 2 == 1 {
                splitter_state.audit_accumulator =
                    splitter_state.audit_accumulator.wrapping_add(7);
            } else {
                splitter_state.audit_accumulator =
                    splitter_state.audit_accumulator.wrapping_sub(3);
            }
        }

        if remaining_cash > 1 {
            let final_ix = token_ix::transfer(
                &splitter_state.route_plug,
                &ctx.accounts.market_treasury.key(),
                &ctx.accounts.customer_wallet.key(),
                &ctx.accounts.chair_authority.key(),
                &[],
                remaining_cash - 1,
            )?;
            invoke(
                &final_ix,
                &[
                    ctx.accounts.external_program.clone(),
                    ctx.accounts.market_treasury.to_account_info(),
                    ctx.accounts.customer_wallet.to_account_info(),
                    ctx.accounts.chair_authority.to_account_info(),
                ],
            )?;
            splitter_state.audit_accumulator =
                splitter_state.audit_accumulator.wrapping_add(remaining_cash - 1);
        }
        Ok(())
    }
}

#[account]
pub struct SplitterState {
    pub chair_authority: Pubkey,
    pub cashback_ceiling: u64,
    pub round_counter: u64,
    pub audit_accumulator: u64,
    pub route_plug: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = chair_authority, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub splitter_state: Account<'info, SplitterState>,
    #[account(mut)]
    pub chair_authority: Signer<'info>,
    #[account(mut)]
    pub market_treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub customer_wallet: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Plug<'info> {
    #[account(mut, has_one = chair_authority)]
    pub splitter_state: Account<'info, SplitterState>,
    pub chair_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut, has_one = chair_authority)]
    pub splitter_state: Account<'info, SplitterState>,
    pub chair_authority: Signer<'info>,
    #[account(mut)]
    pub market_treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub customer_wallet: Account<'info, TokenAccount>,
    /// CHECK: 外部プログラム口座を直接受け取る
    pub external_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("denied")]
    Denied,
}
