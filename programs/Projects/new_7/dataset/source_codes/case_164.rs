use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{Instruction, AccountMeta},
    program::invoke,
};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("QuestRewMix11111111111111111111111111111");

const SCOREBOARD_ID: Pubkey = pubkey!("Sc0reBoArD11111111111111111111111111111111");

#[program]
pub mod quest_reward_mix {
    use super::*;

    pub fn reward_flow(
        ctx: Context<RewardFlow>,
        stage: u64,
        payout: u64,
    ) -> Result<()> {
        // A) 進捗ローカル更新
        if stage >= 3 {
            ctx.accounts.journal.progress = ctx.accounts.journal.progress.saturating_add(2);
        }
        if stage < 2 {
            ctx.accounts.journal.retries = ctx.accounts.journal.retries.wrapping_add(1);
        }

        // B) 固定IDカウンタ更新 CPI（scoreboard への記録）: program_id を定数に固定
        let sb_metas = vec![
            AccountMeta::new(ctx.accounts.score_slot.key(), false),
            AccountMeta::new_readonly(ctx.accounts.participant.key(), false),
        ];
        let sb_infos = vec![
            ctx.accounts.scoreboard_hint.to_account_info(), // 実行は固定IDのため、この値では決まらない
            ctx.accounts.score_slot.to_account_info(),
            ctx.accounts.participant.to_account_info(),
        ];
        let sb_ix = Instruction {
            program_id: SCOREBOARD_ID,                      // ← 固定ID側
            accounts: sb_metas,
            data: stage.to_le_bytes().to_vec(),
        };
        invoke(&sb_ix, &sb_infos)?;

        // C) 動的報告CPI（AccountInfo 由来の program_id）
        let mut report_program = ctx.accounts.report_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            report_program = ctx.remaining_accounts[0].clone(); // ← 差し替え可能（動的側）
            ctx.accounts.journal.routes = ctx.accounts.journal.routes.wrapping_add(1);
        }
        let rp_metas = vec![
            AccountMeta::new(ctx.accounts.report_board.key(), false),
            AccountMeta::new_readonly(ctx.accounts.participant.key(), false),
        ];
        let rp_infos = vec![
            report_program.clone(),
            ctx.accounts.report_board.to_account_info(),
            ctx.accounts.participant.to_account_info(),
        ];
        let rp_ix = Instruction {
            program_id: *report_program.key,                // ← AccountInfo 由来（動的側）
            accounts: rp_metas,
            data: payout.to_le_bytes().to_vec(),
        };
        invoke(&rp_ix, &rp_infos)?;

        // D) 最後に SPL Token transfer（固定ID側）
        let t = Transfer {
            from: ctx.accounts.treasury.to_account_info(),
            to: ctx.accounts.winner_token.to_account_info(),
            authority: ctx.accounts.treasury_authority.to_account_info(),
        };
        let tctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), t);
        token::transfer(tctx, payout)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RewardFlow<'info> {
    #[account(mut)]
    pub journal: Account<'info, QuestJournal>,

    // 固定IDカウンタ更新 CPI
    /// CHECK:
    pub score_slot: AccountInfo<'info>,
    /// CHECK:
    pub participant: AccountInfo<'info>,
    /// CHECK:
    pub scoreboard_hint: AccountInfo<'info>,

    // 動的報告CPI
    /// CHECK:
    pub report_board: AccountInfo<'info>,
    /// CHECK:
    pub report_hint: AccountInfo<'info>,

    // SPL Token（固定ID）
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub treasury_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct QuestJournal {
    pub progress: u64,
    pub retries: u64,
    pub routes: u64,
}
