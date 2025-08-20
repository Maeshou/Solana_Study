// case_002.rs
use anchor_lang::prelude::*;

declare_id!("Safe000000000000000000000000000000000000002");

#[program]
pub mod insecure_reset_settings_v2 {
    use super::*;

    pub fn reset_settings(ctx: Context<ResetSettings>) -> Result<()> {
        let target = &mut ctx.accounts.user_settings;
        let resetter = &mut ctx.accounts.resetter_settings;

        // テーマを強制的に変更
        target.theme = "light".to_string();
        // 言語設定をリセット元からコピー
        target.language = resetter.language.clone();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ResetSettings<'info> {
    // 両方とも mut で受け取り、同一かどうかのチェックなし → Duplicate Mutable Account
    #[account(mut)]
    pub user_settings: Account<'info, Settings>,
    #[account(mut)]
    pub resetter_settings: Account<'info, Settings>,
    // Signer 型がないため、署名チェックが一切行われない → missing_signer
}

#[account]
pub struct Settings {
    pub theme: String,
    pub language: String,
}
