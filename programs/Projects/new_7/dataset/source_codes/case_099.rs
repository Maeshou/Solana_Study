// 9) atelier_pass_pipeline: AccountInfo で外部プログラムを直受け、ブランチ＋ネスト増量
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("AtelierPassP1peline11111111111111111111");

#[program]
pub mod atelier_pass_pipeline {
    use super::*;

    pub fn init(ctx: Context<Init>, bar: u64) -> Result<()> {
        let pipe_state = &mut ctx.accounts.pipe_state;
        pipe_state.admin = ctx.accounts.admin.key();
        pipe_state.bar = bar;
        pipe_state.pace = 7;
        pipe_state.gauge = bar.rotate_left(4);
        pipe_state.route = Pubkey::new_from_array([5u8; 32]);
        Ok(())
    }

    pub fn set(ctx: Context<Set>, p: Pubkey) -> Result<()> {
        let pipe_state = &mut ctx.accounts.pipe_state;
        require_keys_eq!(pipe_state.admin, ctx.accounts.admin.key(), PipeError::AdminOnly);
        pipe_state.route = p;
        pipe_state.pace = pipe_state.pace.saturating_add(2);
        Ok(())
    }

    pub fn pass(ctx: Context<Pass>, amount: u64, turns: u8) -> Result<()> {
        let pipe_state = &mut ctx.accounts.pipe_state;

        if amount == 2 {
            pipe_state.gauge = pipe_state.gauge ^ 0x7A;
            let mut t: u8 = 1;
            while t < 3 {
                pipe_state.pace = pipe_state.pace.saturating_add(1);
                t = t.saturating_add(1);
            }
            return Ok(());
        }

        let mut remaining = amount;
        let mut k: u8 = 0;
        while k < turns {
            let part = (remaining / 2).max(3);
            if part >= remaining {
                break;
            }
            let ix = token_ix::transfer(
                &pipe_state.route,
                &ctx.accounts.storage.key(),
                &ctx.accounts.recipient.key(),
                &ctx.accounts.admin.key(),
                &[],
                part,
            )?;
            // 外部プログラムは AccountInfo で直接受ける
            invoke(
                &ix,
                &[
                    ctx.accounts.external_program.clone(),
                    ctx.accounts.storage.to_account_info(),
                    ctx.accounts.recipient.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;

            remaining = remaining.saturating_sub(part);
            pipe_state.pace = pipe_state.pace.saturating_add(1);
            pipe_state.gauge = pipe_state.gauge.wrapping_add(part ^ 0x08);

            // ブランチ＋ネスト
            if pipe_state.pace % 2 == 0 {
                pipe_state.gauge = pipe_state.gauge.rotate_left(1).wrapping_add(5);
                let mut inner: u8 = 1;
                while inner < 3 {
                    pipe_state.gauge = pipe_state.gauge.wrapping_add(inner as u64);
                    inner = inner.saturating_add(1);
                }
            } else {
                pipe_state.gauge = pipe_state.gauge.rotate_right(2);
            }

            k = k.saturating_add(1);
        }

        if remaining > 3 {
            let ix2 = token_ix::transfer(
                &pipe_state.route,
                &ctx.accounts.storage.key(),
                &ctx.accounts.recipient.key(),
                &ctx.accounts.admin.key(),
                &[],
                remaining - 3,
            )?;
            invoke(
                &ix2,
                &[
                    ctx.accounts.external_program.clone(),
                    ctx.accounts.storage.to_account_info(),
                    ctx.accounts.recipient.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;
            pipe_state.gauge = pipe_state.gauge.wrapping_add(remaining - 3);
        }
        Ok(())
    }
}

#[account]
pub struct PipeState {
    pub admin: Pubkey,
    pub bar: u64,
    pub pace: u64,
    pub gauge: u64,
    pub route: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub pipe_state: Account<'info, PipeState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub storage: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, has_one = admin)]
    pub pipe_state: Account<'info, PipeState>,
    pub admin: Signer<'info>,
}
#[derive(Accounts)]
pub struct Pass<'info> {
    #[account(mut, has_one = admin)]
    pub pipe_state: Account<'info, PipeState>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub storage: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    /// CHECK: 外部プログラム口座
    pub external_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum PipeError {
    #[msg("admin only")]
    AdminOnly,
}
