use anchor_lang::prelude::*;

declare_id!("Ex4000000000000000000000000000000000004");

#[program]
pub mod example4 {
    use super::*;

    // ユーザーを登録
    pub fn register_user(ctx: Context<RegisterUser>, email: String) -> Result<()> {
        let u = &mut ctx.accounts.user;              // ← initあり
        u.email = email;
        u.plan = 0;
        Ok(())
    }

    // プランをアップグレード
    pub fn upgrade_plan(ctx: Context<UpgradePlan>, tier: u8) -> Result<()> {
        let usr = &mut ctx.accounts.user;            // ← initなし：既存参照のみ
        if tier > 0 {
            usr.plan = tier as u32;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(init, payer = admin, space = 8 + 128 + 4)]
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
}
