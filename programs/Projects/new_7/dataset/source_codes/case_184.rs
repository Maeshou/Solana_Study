// 3) rail_report_mix.rs
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("RaIlRePoRtMiX11111111111111111111111111");

const FIXED_LOG_ID: Pubkey = pubkey!("FiXeDLoG00000000000000000000000000000000");

#[program]
pub mod rail_report_mix {
    use super::*;

    fn send_tip(tp: &Program<Token>, bank: &Account<TokenAccount>, rider: &Account<TokenAccount>, auth: &AccountInfo, tip: u64) -> Result<()> {
        token::transfer(
            CpiContext::new(tp.to_account_info(), Transfer {
                from: bank.to_account_info(), to: rider.to_account_info(), authority: auth.clone()
            }),
            tip
        )
    }

    pub fn travel(ctx: Context<Travel>, steps: u64, tip: u64) -> Result<()> {
        if steps > 0 {
            ctx.accounts.trail.progress = ctx.accounts.trail.progress.saturating_add(steps);
        }

        // 固定ID
        let ix_fixed = Instruction {
            program_id: FIXED_LOG_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.step_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.rider.key(), false),
            ],
            data: steps.to_le_bytes().to_vec(),
        };
        invoke(&ix_fixed, &[
            ctx.accounts.step_hint.to_account_info(),
            ctx.accounts.step_cell.to_account_info(),
            ctx.accounts.rider.to_account_info(),
        ])?;

        // 動的CPI
        let mut route = ctx.accounts.cast_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            route = ctx.remaining_accounts[0].clone();
            ctx.accounts.trail.paths = ctx.accounts.trail.paths.wrapping_add(1);
        }
        let ix_dyn = Instruction {
            program_id: *route.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.stage_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.rider.key(), false),
            ],
            data: tip.rotate_right(2).to_le_bytes().to_vec(),
        };
        invoke(&ix_dyn, &[
            route,
            ctx.accounts.stage_board.to_account_info(),
            ctx.accounts.rider.to_account_info(),
        ])?;

        send_tip(&ctx.accounts.token_program, &ctx.accounts.bank, &ctx.accounts.rider_token, &ctx.accounts.bank_authority, tip)
    }
}

#[derive(Accounts)]
pub struct Travel<'info> {
    #[account(mut)]
    pub trail: Account<'info, Trail>,
    /// CHECK:
    pub step_cell: AccountInfo<'info>,
    /// CHECK:
    pub rider: AccountInfo<'info>,
    /// CHECK:
    pub step_hint: AccountInfo<'info>,
    /// CHECK:
    pub stage_board: AccountInfo<'info>,
    /// CHECK:
    pub cast_hint: AccountInfo<'info>,
    #[account(mut)]
    pub bank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub rider_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub bank_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct Trail { pub progress: u64, pub paths: u64 }
