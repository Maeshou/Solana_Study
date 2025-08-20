use anchor_lang::prelude::*;
declare_id!("CaseC333333333333333333333333333333333333333");

#[program]
pub mod account_freezer {
    // 指定アカウントを凍結／解除する関数
    pub fn toggle_freeze(ctx: Context<ToggleFreeze>) -> Result<()> {
        // 本体ロジックはそのまま
        let target = &mut ctx.accounts.target_acc;
        let data = target.try_borrow_mut_data()?;
        data[0] ^= 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ToggleFreeze<'info> {
    /// Signer チェックをアカウント属性で実施
    #[account(signer)]
    pub admin: UncheckedAccount<'info>,

    #[account(mut)]
    pub target_acc: AccountInfo<'info>,
}