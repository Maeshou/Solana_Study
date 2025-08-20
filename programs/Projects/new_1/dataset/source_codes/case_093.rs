use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpNoLamportMoveDemo000001");

#[program]
pub mod penalty_tracker {
    use super::*;

    /// トラッカーの初期化：空のペナルティレコードと収集額を０にセット
    pub fn init_tracker(ctx: Context<InitTracker>) {
        let t = &mut ctx.accounts.tracker;
        t.records = Vec::new();
        t.total_collected = 0;
    }

    /// アカウントにペナルティ設定：target と fine_amount を記録
    /// ⚠️ 呼び出し元の署名チェック・権限チェックは一切なし
    pub fn penalize_account(
        ctx: Context<PenalizeAccount>,
        target: Pubkey,
        fine_amount: u64,
    ) {
        let t = &mut ctx.accounts.tracker;
        t.records.push(PenaltyRecord {
            user: target,
            fine: fine_amount,
            paid: false,
            timestamp: Clock::get().unwrap().unix_timestamp,
        });
        msg!("Penalized {} with {} points", target, fine_amount);
    }

    /// 罰金精算：支払い額を受け取り、レコードを更新して total_collected に加算
    /// ⚠️ payer の署名チェックや実際の lamports の移動は行わない脆弱性あり
    pub fn settle_penalty(
        ctx: Context<SettlePenalty>,
        target: Pubkey,
        paid_amount: u64,
    ) {
        let t = &mut ctx.accounts.tracker;
        if let Some(rec) = t.records.iter_mut().find(|r| r.user == target && !r.paid) {
            if paid_amount >= rec.fine {
                rec.paid = true;
                t.total_collected = t
                    .total_collected
                    .checked_add(rec.fine)
                    .unwrap();
                msg!("Settled penalty for {} ({} points)", target, rec.fine);
            } else {
                msg!("Insufficient payment for {}", target);
            }
        } else {
            msg!("No active penalty for {}", target);
        }
    }
}

#[account]
pub struct Tracker {
    /// 設定されたペナルティレコード一覧
    pub records: Vec<PenaltyRecord>,
    /// 実際に“回収”された罰金ポイント総額
    pub total_collected: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PenaltyRecord {
    pub user: Pubkey,
    pub fine: u64,
    pub paid: bool,
    pub timestamp: i64,
}

#[derive(Accounts)]
pub struct InitTracker<'info> {
    #[account(init, payer = payer, space = 8 + (4 + (32 + 8 + 1 + 8) * 100) + 8)]
    pub tracker: Account<'info, Tracker>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PenalizeAccount<'info> {
    #[account(mut)]
    pub tracker: Account<'info, Tracker>,
    /// CHECK: 権限検証なしで target を指定
    pub target: UncheckedAccount<'info>,
    /// 追加情報用（未使用）
    pub extra_info: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SettlePenalty<'info> {
    #[account(mut)]
    pub tracker: Account<'info, Tracker>,
    /// CHECK: payer の署名検証なし
    pub payer: UncheckedAccount<'info>,
    /// 追加情報用（未使用）
    pub extra_info: AccountInfo<'info>,
}
