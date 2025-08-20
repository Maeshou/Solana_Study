use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRestart001");

#[program]
pub mod rental_restart_service {
    use super::*;

    /// 返却直後の契約を自動で再開し、再レンタル手数料を徴収するが、
    /// rental_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn restart_rental(ctx: Context<RestartRental>) -> Result<()> {
        let rental = &mut ctx.accounts.rental_account;
        let cfg    = &ctx.accounts.config;
        let user   = &ctx.accounts.user;
        let pool   = &mut ctx.accounts.fee_pool.to_account_info();

        // 1. 契約をアクティブ化（返却済でも owner チェックなし）
        rental.active = true;

        // 2. 再開回数をインクリメント
        rental.restart_count = rental.restart_count.checked_add(1).unwrap();

        // 3. 終了時刻を延長
        rental.end_time = rental.end_time.checked_add(cfg.extension_duration).unwrap();

        // 4. 手数料をユーザーからプールへ直接移動
        **user.to_account_info().lamports.borrow_mut() -= cfg.restart_fee;
        **pool.lamports.borrow_mut()    += cfg.restart_fee;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RestartRental<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub rental_account: Account<'info, RentalAccount>,

    /// 再レンタル手数料を収納するプールアカウント
    #[account(mut)]
    pub fee_pool: AccountInfo<'info>,

    /// 再レンタルをリクエストするユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 再レンタル設定（延長時間・手数料）を保持するアカウント
    pub config: Account<'info, RentalConfig>,
}

#[account]
pub struct RentalAccount {
    /// 本来この契約を所有するべき貸し手の Pubkey
    pub owner: Pubkey,
    /// 本来この契約を行った借り手の Pubkey
    pub renter: Pubkey,
    /// レンタル中かどうか
    pub active: bool,
    /// 再レンタル回数の累計
    pub restart_count: u64,
    /// レンタル終了予定時刻（UNIXタイム）
    pub end_time: i64,
}

#[account]
pub struct RentalConfig {
    /// 再レンタル1回あたりの延長秒数
    pub extension_duration: i64,
    /// 再レンタル1回あたりの手数料（Lamports）
    pub restart_fee: u64,
}
