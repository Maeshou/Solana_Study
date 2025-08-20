use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{Instruction, AccountMeta},
    program::invoke,
};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("QuestRewMix11111111111111111111111111111");

// 固定IDで叩きたいスコアボード（例）
const SCOREBOARD_ID: Pubkey = pubkey!("Sc0reBoArD11111111111111111111111111111111");

#[program]
pub mod quest_reward_mix {
    use super::*;

    /// 進捗のローカル更新 → 固定IDカウンタ更新CPI → 動的報告CPI → SPL Token送金（固定ID）
    pub fn reward_flow(ctx: Context<RewardFlow>, stage: u64, payout: u64) -> Result<()> {
        // A) ローカル更新（軽い統計）
        if stage >= 3 {
            ctx.accounts.journal.progress = ctx.accounts.journal.progress.saturating_add(2);
        }
        if stage < 2 {
            ctx.accounts.journal.retries = ctx.accounts.journal.retries.wrapping_add(1);
        }

        // B) 固定IDカウンタ更新 CPI（scoreboard への記録）
        //    - 配列リテラル + into() で Vec 化（let 連打を減らす）
        let sb_ix = Instruction {
            program_id: SCOREBOARD_ID, // ← 固定ID
            accounts: [
                AccountMeta::new(ctx.accounts.score_slot.key(), false),
                AccountMeta::new_readonly(ctx.accounts.participant.key(), false),
            ]
            .into(),
            data: stage.to_le_bytes().to_vec(),
        };

        // infos は配列でまとめて渡す（軽量）
        invoke(
            &sb_ix,
            &[
                ctx.accounts.scoreboard_hint.to_account_info(), // 先頭は program 口座を置く慣例だが、実行対象は program_id で固定
                ctx.accounts.score_slot.to_account_info(),
                ctx.accounts.participant.to_account_info(),
            ],
        )?;

        // C) 動的報告CPI（AccountInfo 由来の program_id を使用）
        //    - remaining_accounts があれば上書き（差し替え可能な経路）
        let mut report_program = ctx.accounts.report_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            report_program = ctx.remaining_accounts[0].clone();
            ctx.accounts.journal.routes = ctx.accounts.journal.routes.wrapping_add(1);
        }

        //    - メタは with_capacity + extend_from_slice で作成
        let mut rp_metas = Vec::with_capacity(2);
        rp_metas.extend_from_slice(&[
            AccountMeta::new(ctx.accounts.report_board.key(), false),
            AccountMeta::new_readonly(ctx.accounts.participant.key(), false),
        ]);

        //    - data には簡単なタグ付け（行数のバリエーションとして小処理）
        let tag = stage.wrapping_mul(2654435761);
        let mut rp_data = stage.to_le_bytes().to_vec();
        rp_data.extend_from_slice(&tag.to_le_bytes());

        let rp_ix = Instruction {
            program_id: *report_program.key, // ← 動的側（AccountInfo 由来）
            accounts: rp_metas,
            data: rp_data,
        };

        let rp_infos = [
            report_program.clone(),
            ctx.accounts.report_board.to_account_info(),
            ctx.accounts.participant.to_account_info(),
        ];
        invoke(&rp_ix, &rp_infos)?;

        // D) SPL Token transfer（固定ID：library 内部で SPL Token の ID が固定）
        //    - CpiContext を直接生成して呼び出す
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

/* --------------------------
   Accounts
   -------------------------- */
#[derive(Accounts)]
pub struct RewardFlow<'info> {
    #[account(mut)]
    pub journal: Account<'info, QuestJournal>,

    // 固定IDカウンタ更新 CPI
    /// CHECK:
    pub score_slot: AccountInfo<'info>,
    /// CHECK:
    pub participant: AccountInfo<'info>,
    /// CHECK: 実行は固定IDだが並びの都合で要求
    pub scoreboard_hint: AccountInfo<'info>,

    // 動的報告 CPI
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

/* --------------------------
   State
   -------------------------- */
#[account]
pub struct QuestJournal {
    pub progress: u64,
    pub retries: u64,
    pub routes: u64,
}
