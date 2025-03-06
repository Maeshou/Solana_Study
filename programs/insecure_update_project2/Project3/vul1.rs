#[program]
pub mod data_validation {
    use super::*;

    pub fn update_admin(ctx: Context<UpdateAdmin>) -> Result<()> {
        // 脆弱性: `admin` フィールドは署名の検証なしで更新される
        ctx.accounts.admin_config.admin = ctx.accounts.new_admin.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    #[account(mut)]
    pub admin_config: Account<'info, AdminConfig>,
    #[account(mut)]
    pub admin: Signer<'info>, // 現在の管理者アカウント
    pub new_admin: SystemAccount<'info>, // 新しい管理者アカウント
}

#[account]
pub struct AdminConfig {
    pub admin: Pubkey, // 管理者の公開鍵
}
