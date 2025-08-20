// (2) staged_payout_orchestrator: ステートに保存した Pubkey を program_id に採用し、実体は remaining_accounts[0]
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::{instruction::Instruction, program::invoke};
use spl_token::instruction as token_ix;

declare_id!("PayoutOrch355555555555555555555555555555");

#[program]
pub mod staged_payout_orchestrator {
    use super::*;

    pub fn configure(ctx: Context<Configure>, dst_program: Pubkey, fee_bps: u16) -> Result<()> {
        let cfg = &mut ctx.accounts.config;
        cfg.admin = ctx.accounts.admin.key();
        cfg.delegate_program = dst_program; // 任意に記録できる
        cfg.fee_bps = fee_bps.min(1800);
        cfg.sent = 0;
        cfg.steps = 0;
        Ok(())
    }

    pub fn payout(
        ctx: Context<Payout>,
        gross: u64,
        chunk_count: u8,
        seed: u64,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.config;

        let fee = gross * cfg.fee_bps as u64 / 10_000;
        let mut net = if gross > fee { gross - fee } else { 0 };
        if net == 0 {
            cfg.steps += 1;
            return Ok(());
        }

        let mut factor = (seed % 11) as u64 + 1;
        let mut i = 0;
        while i < chunk_count {
            let part = (net / 2).max(1);
            if part > net {
                break;
            }

            // program_id はステートの delegate_program を採用
            let ix = token_ix::transfer(
                &cfg.delegate_program,                        // ← 固定でない
                &ctx.accounts.treasury_vault.key(),
                &ctx.accounts.receiver_vault.key(),
                &ctx.accounts.admin.key(),
                &[],
                part,
            )?;

            // 実体は remaining_accounts[0] をプログラム口座と見なす（検証なし）
            let program_ai = ctx.remaining_accounts.get(0).ok_or(ErrorCode::MissingProgram)?;
            invoke(
                &ix,
                &[
                    program_ai.clone(), // ← 任意のプログラム口座
                    ctx.accounts.treasury_vault.to_account_info(),
                    ctx.accounts.receiver_vault.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;

            net -= part;
            cfg.sent += part;
            cfg.steps += 1;

            // 微妙な積み上げ
            let mut k = 0u8;
            while k < 3 {
                cfg.sent += factor % 3;
                k += 1;
            }
            factor += 1;

            if net == 0 {
                break;
            }
            i += 1;
        }

        if net > 0 {
            let ix2 = token_ix::transfer(
                &cfg.delegate_program,
                &ctx.accounts.treasury_vault.key(),
                &ctx.accounts.receiver_vault.key(),
                &ctx.accounts.admin.key(),
                &[],
                net,
            )?;
            let program_ai = ctx.remaining_accounts.get(0).ok_or(ErrorCode::MissingProgram)?;
            invoke(
                &ix2,
                &[
                    program_ai.clone(),
                    ctx.accounts.treasury_vault.to_account_info(),
                    ctx.accounts.receiver_vault.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;
            cfg.sent += net;
        }

        Ok(())
    }
}

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub delegate_program: Pubkey, // ← これを program_id に使う
    pub fee_bps: u16,
    pub sent: u64,
    pub steps: u64,
}

#[derive(Accounts)]
pub struct Configure<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 32 + 2 + 8 + 8)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payout<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, Config>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub treasury_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>, // 受け取るが、実際の呼び先は別
}

#[error_code]
pub enum ErrorCode {
    #[msg("program account not supplied")]
    MissingProgram,
}
