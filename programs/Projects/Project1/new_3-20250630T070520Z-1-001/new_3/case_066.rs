use anchor_lang::prelude::*;
use anchor_lang::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgLateFee001");

#[program]
pub mod rental_service {
    use super::*;

    /// 返却期限超過時に遅延ペナルティを課すが、
    /// rental_account.owner と請求ユーザーの照合チェックがない
    pub fn apply_late_fee(ctx: Context<ApplyLateFee>) -> Result<()> {
        let rental = &mut ctx.accounts.rental_account;
        let config = &ctx.accounts.config;
        let user = &mut ctx.accounts.user;
        let pool = &mut ctx.accounts.penalty_pool.to_account_info();

        // 1. 現在の UNIX タイムスタンプを取得
        let now = Clock::get()?.unix_timestamp;

        // 2. 返却期限超過秒数を計算
        let overdue = (now - rental.end_time).max(0) as u64;

        // 3. 遅延ペナルティ額を計算 (超過秒数 × 単価)
        let penalty = overdue
            .checked_mul(config.late_fee_per_second)
            .unwrap();

        // 4. 累計遅延ペナルティを更新
        rental.total_late_fees = rental.total_late_fees.checked_add(penalty).unwrap();

        // 5. ペナルティをユーザーからプールへ直接送金
        **user.to_account_info().lamports.borrow_mut() -= penalty;
        **pool.lamports.borrow_mut()      += penalty;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ApplyLateFee<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して
    /// rental_account.owner と請求ユーザーの一致検証を行うべき
    pub rental_account: Account<'info, RentalAccount>,

    /// 遅延ペナルティを収納するプールアカウント
    #[account(mut)]
    pub penalty_pool: AccountInfo<'info>,

    /// ペナルティを支払うユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 遅延単価を保持する設定アカウント
    pub config: Account<'info, RentalConfig>,
}

#[account]
pub struct RentalAccount {
    /// 本来この契約を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// レンタル終了予定時刻 (UNIXタイム)
    pub end_time: i64,
    /// 累計で課された遅延ペナルティ (Lamports)
    pub total_late_fees: u64,
}

#[account]
pub struct RentalConfig {
    /// 遅延 1 秒あたりのペナルティ (Lamports)
    pub late_fee_per_second: u64,
}
