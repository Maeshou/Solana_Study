// 5) raid_reward_switchboard（全面リネーム版）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Ra1dRewardSw1tchb0ard1111111111111111111");

#[program]
pub mod raid_reward_switchboard {
    use super::*;

    pub fn init(ctx: Context<Init>, hardcap: u64) -> Result<()> {
        let switch_state = &mut ctx.accounts.switch_state;
        switch_state.manager_authority = ctx.accounts.manager_authority.key();
        switch_state.hardcap_limit = hardcap;
        switch_state.wave_counter = 4;
        switch_state.trace_accumulator = hardcap ^ 0xDEAD_BEEF;
        Ok(())
    }

    pub fn approve(ctx: Context<Approve>, prog: Pubkey) -> Result<()> {
        let switch_state = &mut ctx.accounts.switch_state;
        require_keys_eq!(
            switch_state.manager_authority,
            ctx.accounts.manager_authority.key(),
            ErrorCode::Denied
        );
        switch_state.route_program_id = prog;
        switch_state.wave_counter = switch_state.wave_counter.wrapping_add(5);
        Ok(())
    }

    pub fn payout(ctx: Context<Payout>, prize: u64, pass: u8) -> Result<()> {
        let switch_state = &mut ctx.accounts.switch_state;

        if prize <= 2 {
            switch_state.wave_counter = switch_state.wave_counter.wrapping_add(9);
            switch_state.trace_accumulator = switch_state.trace_accumulator.rotate_left(4);
            return Ok(());
        }

        let mut remaining_prize = prize;
        let mut step_index: u8 = 1;
        while step_index <= pass {
            let unit = (remaining_prize / 3).max(3);
            if unit >= remaining_prize {
                break;
            }

            let transfer_ix = token_ix::transfer(
                &switch_state.route_program_id,
                &ctx.accounts.bank_vault.key(),
                &ctx.accounts.winner_vault.key(),
                &ctx.accounts.manager_authority.key(),
                &[],
                unit,
            )?;
            let external_program_ai =
                ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &transfer_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.bank_vault.to_account_info(),
                    ctx.accounts.winner_vault.to_account_info(),
                    ctx.accounts.manager_authority.to_account_info(),
                ],
            )?;

            remaining_prize = remaining_prize.saturating_sub(unit);
            switch_state.wave_counter = switch_state.wave_counter.saturating_add(1);
            switch_state.trace_accumulator = switch_state
                .trace_accumulator
                .wrapping_add(unit as u64)
                .rotate_right(2);
            step_index = step_index.saturating_add(1);

            if remaining_prize < switch_state.hardcap_limit / 5 {
                switch_state.trace_accumulator = switch_state.trace_accumulator ^ 0xABCD;
            } else {
                switch_state.trace_accumulator =
                    switch_state.trace_accumulator.wrapping_add(31);
            }
        }

        if remaining_prize > 2 {
            let final_ix = token_ix::transfer(
                &switch_state.route_program_id,
                &ctx.accounts.bank_vault.key(),
                &ctx.accounts.winner_vault.key(),
                &ctx.accounts.manager_authority.key(),
                &[],
                remaining_prize - 2,
            )?;
            let external_program_ai =
                ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &final_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.bank_vault.to_account_info(),
                    ctx.accounts.winner_vault.to_account_info(),
                    ctx.accounts.manager_authority.to_account_info(),
                ],
            )?;
            switch_state.trace_accumulator =
                switch_state.trace_accumulator.wrapping_add(remaining_prize - 2);
        }
        Ok(())
    }
}

#[account]
pub struct SwitchState {
    pub manager_authority: Pubkey,
    pub hardcap_limit: u64,
    pub wave_counter: u64,
    pub trace_accumulator: u64,
    pub route_program_id: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = manager_authority, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub switch_state: Account<'info, SwitchState>,
    #[account(mut)]
    pub manager_authority: Signer<'info>,
    #[account(mut)]
    pub bank_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(mut, has_one = manager_authority)]
    pub switch_state: Account<'info, SwitchState>,
    pub manager_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Payout<'info> {
    #[account(mut, has_one = manager_authority)]
    pub switch_state: Account<'info, SwitchState>,
    pub manager_authority: Signer<'info>,
    #[account(mut)]
    pub bank_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("no program")]
    NoProgram,
    #[msg("denied")]
    Denied,
}
