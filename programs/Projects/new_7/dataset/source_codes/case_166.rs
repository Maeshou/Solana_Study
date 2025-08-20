use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Mix01QuestAward111111111111111111111111111");

const SCOREBOARD_ID_A: Pubkey = pubkey!("Sc0reBoArdAAAA1111111111111111111111111111");

#[program]
pub mod quest_award_and_report {
    use super::*;

    pub fn run(ctx: Context<Run>, stage: u64, amount: u64) -> Result<()> {
        // 固定ID（安全寄り）：スコアボード更新
        let fixed_ix = Instruction {
            program_id: SCOREBOARD_ID_A,
            accounts: vec![
                AccountMeta::new(ctx.accounts.slot_counter.key(), false),
                AccountMeta::new_readonly(ctx.accounts.player.key(), false),
            ],
            data: stage.to_le_bytes().to_vec(),
        };
        invoke(
            &fixed_ix,
            &[
                ctx.accounts.scoreboard_marker.to_account_info(),
                ctx.accounts.slot_counter.to_account_info(),
                ctx.accounts.player.to_account_info(),
            ],
        )?;

        // 動的CPI（program_id を AccountInfo 由来で決定）
        let mut target_program = ctx.accounts.report_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            target_program = ctx.remaining_accounts[0].clone();
            ctx.accounts.memo.routes = ctx.accounts.memo.routes.saturating_add(1);
        }
        let dynamic_ix = Instruction {
            program_id: *target_program.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.report_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.player.key(), false),
            ],
            data: amount.to_le_bytes().to_vec(),
        };
        invoke(
            &dynamic_ix,
            &[
                target_program,
                ctx.accounts.report_board.to_account_info(),
                ctx.accounts.player.to_account_info(),
            ],
        )?;

        // SPL Token transfer（安全寄り：内部で Token Program ID 固定）
        let t = Transfer {
            from: ctx.accounts.treasury.to_account_info(),
            to: ctx.accounts.recipient_token.to_account_info(),
            authority: ctx.accounts.treasury_authority.to_account_info(),
        };
        let tctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), t);
        token::transfer(tctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut)]
    pub memo: Account<'info, LocalMemo>,
    /// CHECK:
    pub slot_counter: AccountInfo<'info>,
    /// CHECK:
    pub player: AccountInfo<'info>,
    /// CHECK:
    pub scoreboard_marker: AccountInfo<'info>,
    /// CHECK:
    pub report_board: AccountInfo<'info>,
    /// CHECK:
    pub report_hint: AccountInfo<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub treasury_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct LocalMemo {
    pub routes: u64,
}
