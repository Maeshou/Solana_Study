use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgqSubscrUpd1");

#[program]
pub mod insecure_subscription {
    use super::*;

    /// サブスクリプションの有効/無効を切り替えるインストラクション
    pub fn toggle_subscription(ctx: Context<ToggleSubscription>) -> Result<()> {
        let acct_info = &mut ctx.accounts.subscription_account.to_account_info();

        // ★ オーナーチェック（owner == program_id）を行っていない！
        //    別プログラム所有のアカウントでもここを通過してしまう

        // アカウントデータの先頭バイトを「有効フラグ」として反転
        let data = &mut acct_info.data.borrow_mut();
        let flag = data.get(0).copied().unwrap_or(0);
        data[0] = if flag == 0 { 1 } else { 0 };

        msg!("Subscription active flag is now: {}", data[0]);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ToggleSubscription<'info> {
    /// CHECK: owner フィールドの検証をしていない生の AccountInfo
    #[account(mut)]
    pub subscription_account: AccountInfo<'info>,

    /// 実行者が署名していることのみを検証
    pub signer: Signer<'info>,
}
