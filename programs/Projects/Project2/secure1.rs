use anchor_lang::prelude::*;

#[program]
pub mod owner_check {
    use super::*;

    pub fn admin_instruction(ctx: Context<Checked>) -> Result<()> {
        // AdminConfig内のadminフィールドの値を表示
        msg!("Admin: {}", ctx.accounts.admin_config.admin.to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Checked<'info> {
    // admin_configアカウントの所有権を確認
    #[account(
        has_one = admin,
    )]
    pub admin_config: Account<'info, AdminConfig>,

    // adminフィールドが署名者であることを確認
    pub admin: Signer<'info>,
}

#[account]
pub struct AdminConfig {
    pub admin: Pubkey, // 管理者の公開鍵
}
