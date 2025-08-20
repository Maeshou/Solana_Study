use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgExtRntl0001");

#[program]
pub mod rental_extension_service {
    use super::*;

    /// レンタル期間を延長し、延長料金を徴収するが、
    /// rental_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn extend_rental(ctx: Context<ExtendRental>, extra_time: i64) -> Result<()> {
        let rental = &mut ctx.accounts.rental_account;
        let cfg = &ctx.accounts.config;

        // 延長料金を計算 (秒数 × 単価)
        let fee = (extra_time as u64)
            .checked_mul(cfg.fee_per_second)
            .unwrap();

        // ユーザーからプールへ Lamports を移動
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() -= fee;
        **ctx.accounts.fee_pool.to_account_info().lamports.borrow_mut() += fee;

        // レンタル終了時刻を延長
        rental.end_time = rental.end_time.checked_add(extra_time).unwrap();

        // 延長回数をインクリメント
        rental.extension_count = rental.extension_count.checked_add(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExtendRental<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub rental_account: Account<'info, RentalAccount>,

    /// 延長料金を集めるプールアカウント
    #[account(mut)]
    pub fee_pool: AccountInfo<'info>,

    /// 延長をリクエストするユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 延長料金単価を保持する設定アカウント
    pub config: Account<'info, RentalConfig>,
}

#[account]
pub struct RentalAccount {
    /// 本来この契約を所有するべき貸し手の Pubkey
    pub owner: Pubkey,
    /// 本来この契約を行った借り手の Pubkey
    pub renter: Pubkey,
    /// レンタル開始時刻 (UNIXタイム)
    pub start_time: i64,
    /// レンタル終了時刻 (UNIXタイム)
    pub end_time: i64,
    /// これまでの延長回数
    pub extension_count: u64,
}

#[account]
pub struct RentalConfig {
    /// 延長 1 秒あたりの料金 (Lamports)
    pub fee_per_second: u64,
}
