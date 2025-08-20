use anchor_lang::prelude::*;

declare_id!("Reset111111111111111111111111111111111111111");

#[program]
pub mod insecure_reset_settings {
    use super::*;

    pub fn reset_settings(ctx: Context<ResetSettings>) -> Result<()> {
        let settings = &mut ctx.accounts.user_settings;
        // 複数行にわたるリセット処理の例
        settings.flag_a = false;
        settings.flag_b = 0;
        settings.comment.clear();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ResetSettings<'info> {
    #[account(mut)]
    pub user_settings: Account<'info, UserSettings>,
    /// ここで署名者チェックを追加
    pub user: Signer<'info>,
}

#[account]
pub struct UserSettings {
    pub flag_a: bool,
    pub flag_b: u64,
    pub comment: String,
}