use anchor_lang::prelude::*;

declare_id!("EXAMPLEEXAMPLEEXAMPLEEXAMPLEEX");

#[program]
pub mod reinit_misconfigured {
    use super::*;

    /// 全アカウントを初期化するつもりが、settings_account の init が漏れている例
    pub fn init_all(
        ctx: Context<InitAll>,
        mode: u8,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.config_account;
        cfg.mode = mode;
        cfg.version = 1;

        // settings_account は初期化されず、渡された既存アカウントに上書きしてしまう
        let settings = &mut ctx.accounts.settings_account;
        settings.theme = "default".to_string();
        settings.notifications = true;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAll<'info> {
    /// こちらは正しく作成・初期化される
    #[account(init, payer = user, space = 8 + 1 + 1)]
    pub config_account: Account<'info, ConfigData>,

    /// init 属性が抜けているため、本来新規作成すべき SettingsData を既存アカウントで再利用してしまう
    #[account(mut)]
    pub settings_account: Account<'info, SettingsData>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ConfigData {
    pub mode: u8,
    pub version: u8,
}

#[account]
pub struct SettingsData {
    pub theme: String,
    pub notifications: bool,
}
