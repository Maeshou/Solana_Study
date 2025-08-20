use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxNEWPENALTY000000000");

#[program]
pub mod penalty_reset {
    use super::*;

    /// 支払額を累積し、罰金が全額支払われたら自動で復活フラグを立てます。
    /// ──────────────────────────────────
    /// `payment`：今回支払う罰金額
    pub fn settle_penalty(ctx: Context<SettlePenalty>, payment: u64) {
        // Contextからアカウント構造体を一度だけ取り出す
        let SettlePenalty { penalty_record, .. } = &mut ctx.accounts;

        // saturating_addでパニックなしに加算
        penalty_record.fine_paid = penalty_record
            .fine_paid
            .saturating_add(payment);

        // 復活判定は比較式で直接ブール値を得る
        penalty_record.is_active = penalty_record.fine_paid
            .checked_ge(&penalty_record.penalty_due)
            .unwrap_or(false);
    }
}

#[derive(Accounts)]
pub struct SettlePenalty<'info> {
    /// 手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:       Signer<'info>,

    /// 脆弱性として署名チェックを省いたユーザー
    pub user:            AccountInfo<'info>,

    /// 罰金情報を保持する PDA
    #[account(
        mut,
        seeds = [b"penalty", user.key().as_ref()],
        bump
    )]
    pub penalty_record:  Account<'info, PenaltyInfo>,

    pub system_program:  Program<'info, System>,
}

#[account]
pub struct PenaltyInfo {
    /// 課せられた罰金総額
    pub penalty_due:     u64,
    /// ユーザーが支払った累計罰金
    pub fine_paid:       u64,
    /// 全額支払い後に true となる復活フラグ
    pub is_active:       bool,
}
