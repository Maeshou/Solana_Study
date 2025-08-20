use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgThemeSrv001");

#[program]
pub mod theme_service {
    use super::*;

    /// ユーザーがプロフィールのテーマカラーとパターンを変更するが、
    /// theme_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn update_theme(
        ctx: Context<UpdateTheme>,
        color: String,
        pattern: String,
    ) -> Result<()> {
        let theme = &mut ctx.accounts.theme_account;

        // ↓ 本来は theme.owner と ctx.accounts.user.key() の一致を検証すべき
        theme.color = color;
        theme.pattern = pattern;
        theme.change_count = theme.change_count.checked_add(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateTheme<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付与して照合チェックを行うべき
    pub theme_account: Account<'info, ThemeAccount>,
    /// テーマを更新するユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct ThemeAccount {
    /// このテーマアカウントを所有すべきユーザーの Pubkey
    pub owner: Pubkey,
    /// プロフィールのテーマカラー（例："#FFAA00"）
    pub color: String,
    /// プロフィールのパターン名（例："stripes"）
    pub pattern: String,
    /// これまでに変更した回数
    pub change_count: u64,
}
