use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgCancelRn001");

#[program]
pub mod rental_service {
    use super::*;

    /// ユーザー都合のレンタルキャンセル処理だが、
    /// rental_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn cancel_rental(ctx: Context<CancelRental>) -> Result<()> {
        let rental = &mut ctx.accounts.rental_account;
        let penalty = ctx.accounts.config.cancel_penalty;

        // 1. レンタル状態を解除
        rental.active = false;

        // 2. キャンセル回数をインクリメント
        rental.cancel_count = rental.cancel_count.checked_add(1).unwrap();

        // 3. ペナルティ金額をユーザーからプールへ直接送金
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() -= penalty;
        **ctx.accounts.penalty_pool.to_account_info().lamports.borrow_mut() += penalty;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CancelRental<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub rental_account: Account<'info, RentalAccount>,

    /// キャンセル料を受け取るプールアカウント
    #[account(mut)]
    pub penalty_pool: AccountInfo<'info>,

    /// キャンセルを実行するユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// キャンセルペナルティ設定を保持するアカウント
    pub config: Account<'info, CancelConfig>,
}

#[account]
pub struct RentalAccount {
    /// 本来このレンタル契約を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 現在レンタル中かどうか
    pub active: bool,
    /// これまでにキャンセルされた回数
    pub cancel_count: u64,
}

#[account]
pub struct CancelConfig {
    /// キャンセル時に徴収するペナルティ額（Lamports）
    pub cancel_penalty: u64,
}
