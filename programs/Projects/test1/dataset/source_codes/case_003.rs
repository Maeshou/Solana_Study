use anchor_lang::prelude::*;
declare_id!("CaseC333333333333333333333333333333333333333");

#[program]
pub mod account_freezer {
    // 指定アカウントを凍結／解除する関数
    pub fn toggle_freeze(ctx: Context<ToggleFreeze>) -> Result<()> {
        // adminのSignerチェックがない
        let target = &mut ctx.accounts.target_acc;
        // ownerチェックしないまま byte[0] を凍結フラグ用に反転
        let data = target.try_borrow_mut_data()?;
        data[0] ^= 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ToggleFreeze<'info> {
    /// CHECK: サイナー検証なし
    pub admin: UncheckedAccount<'info>,
    /// CHECK: このアカウントが想定プログラムに属しているか未検証
    #[account(mut)]
    pub target_acc: AccountInfo<'info>,
}
