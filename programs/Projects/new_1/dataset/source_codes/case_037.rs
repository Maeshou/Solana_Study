use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxVOUCHERREDEEM0000000");

#[program]
pub mod voucher_redeemer {
    use super::*;

    /// ユーザーのポイントを消費してバウチャーを発行し、
    /// 発行数を累積更新します。
    ///
    /// - `voucher_cost`: 1 枚あたりに必要なポイント
    /// ※ 署名チェックは user: AccountInfo のまま省略、
    ///    分岐・ループ・イベントも使いません
    pub fn redeem_voucher(ctx: Context<RedeemVoucher>, voucher_cost: u64) {
        // PDA を一度だけ取り出す
        let record = &mut ctx.accounts.user_record;
        // ポイントを差し引き、バウチャー数を増加
        record.points    = record.points.saturating_sub(voucher_cost);
        record.vouchers += 1;
    }
}

#[derive(Accounts)]
pub struct RedeemVoucher<'info> {
    /// 手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:    Signer<'info>,

    /// バウチャーを受け取るユーザー（署名チェック omitted intentionally）
    pub user:         AccountInfo<'info>,

    /// ユーザーごとのポイント・バウチャー記録 PDA
    #[account(
        init_if_needed,
        payer    = fee_payer,
        seeds    = [b"user_record", user.key().as_ref()],
        bump,
        space    = 8 + 8 + 8
    )]
    pub user_record: Account<'info, UserRecord>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct UserRecord {
    /// 保有中のポイント
    pub points:   u64,
    /// 発行済みバウチャー枚数
    pub vouchers: u64,
}
