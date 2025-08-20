// 5) raid_reward_switchboard: route_id をアカウントから読み取り program_id に流用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Ra1dRewardSw1tchb0ard1111111111111111111");

#[program]
pub mod raid_reward_switchboard {
    use super::*;

    pub fn init(ctx: Context<Init>, hardcap: u64) -> Result<()> {
        let st = &mut ctx.accounts.state;
        st.mgr = ctx.accounts.mgr.key();
        st.hardcap = hardcap;
        st.waves = 4;
        st.trace = hardcap ^ 0xDEAD_BEEF;
        Ok(())
    }

    pub fn approve(ctx: Context<Approve>, prog: Pubkey) -> Result<()> {
        let st = &mut ctx.accounts.state;
        require_keys_eq!(st.mgr, ctx.accounts.mgr.key(), ErrorCode::Denied);
        st.route_id = prog;
        st.waves += 5;
        Ok(())
    }

    pub fn payout(ctx: Context<Payout>, prize: u64, pass: u8) -> Result<()> {
        let st = &mut ctx.accounts.state;

        if prize <= 2 {
            st.waves = st.waves.wrapping_add(9);
            st.trace = st.trace.rotate_left(4);
            return Ok(());
        }

        let mut balance = prize;
        let mut step = 1u8;
        while step <= pass {
            let unit = (balance / 3).max(3);
            if unit >= balance {
                break;
            }

            let ix = token_ix::transfer(
                &st.route_id,
                &ctx.accounts.bank.key(),
                &ctx.accounts.winner.key(),
                &ctx.accounts.mgr.key(),
                &[],
                unit,
            )?;
            let program_ai = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix,
                &[
                    program_ai.clone(),
                    ctx.accounts.bank.to_account_info(),
                    ctx.accounts.winner.to_account_info(),
                    ctx.accounts.mgr.to_account_info(),
                ],
            )?;

            balance -= unit;
            st.waves += 1;
            st.trace = st.trace.wrapping_add(unit as u64).rotate_right(2);
            step += 1;

            if balance < st.hardcap / 5 {
                st.trace = st.trace ^ 0xABCD;
            } else {
                st.trace = st.trace.wrapping_add(31);
            }
        }

        if balance > 2 {
            let ix2 = token_ix::transfer(
                &st.route_id,
                &ctx.accounts.bank.key(),
                &ctx.accounts.winner.key(),
                &ctx.accounts.mgr.key(),
                &[],
                balance - 2,
            )?;
            let program_ai = ctx.remaining_accounts.get(0).ok_or(ErrorCode::NoProgram)?;
            invoke(
                &ix2,
                &[
                    program_ai.clone(),
                    ctx.accounts.bank.to_account_info(),
                    ctx.accounts.winner.to_account_info(),
                    ctx.accounts.mgr.to_account_info(),
                ],
            )?;
            st.trace = st.trace.wrapping_add(balance - 2);
        }
        Ok(())
    }
}

#[account]
pub struct State {
    pub mgr: Pubkey,
    pub hardcap: u64,
    pub waves: u64,
    pub trace: u64,
    pub route_id: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = mgr, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub mgr: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(mut, has_one = mgr)]
    pub state: Account<'info, State>,
    pub mgr: Signer<'info>,
}

#[derive(Accounts)]
pub struct Payout<'info> {
    #[account(mut, has_one = mgr)]
    pub state: Account<'info, State>,
    pub mgr: Signer<'info>,
    #[account(mut)]
    pub bank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("no program")]
    NoProgram,
    #[msg("denied")]
    Denied,
}
