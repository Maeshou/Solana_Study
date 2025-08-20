use anchor_lang::prelude::*;

declare_id!("Ex4000000000000000000000000000000000004");

#[program]
pub mod example4 {
    use super::*;

    // ユーザーを登録し、メール検証とタイムスタンプ記録
    pub fn register_user(ctx: Context<RegisterUser>, email: String) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let u = &mut ctx.accounts.user;            // ← initあり
        u.email = email.clone();
        u.plan = 0;
        u.last_updated = now;
        u.valid_email = email.contains("@");
        Ok(())
    }

    // プランをアップグレードし、更新回数をカウント
    pub fn upgrade_plan(ctx: Context<UpgradePlan>, tier: u8) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let u = &mut ctx.accounts.user;            // ← initなし：既存参照のみ
        if tier > 0 {
            u.plan = tier as u32;
            u.upgrade_count += 1;
            u.last_updated = now;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(init, payer = admin, space = 8 + 128 + 4 + 8*2 + 1)]
    pub user: Account<'info, UserData>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpgradePlan<'info> {
    pub user: Account<'info, UserData>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserData {
    pub email: String,
    pub plan: u32,
    pub last_updated: i64,
    pub valid_email: bool,
    pub upgrade_count: u8,
}
