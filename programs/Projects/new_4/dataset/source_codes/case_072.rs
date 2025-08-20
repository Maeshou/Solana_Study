use anchor_lang::prelude::*;

declare_id!("NextCaseSub222222222222222222222222222222");

#[program]
pub mod example7 {
    use super::*;

    // ユーザー登録（user_account にだけ init）
    pub fn register_user(ctx: Context<RegisterUser>, email: String) -> Result<()> {
        let u = &mut ctx.accounts.user_account;
        u.email = email;
        u.plan = 0;
        Ok(())
    }

    // プラン変更と履歴記録（history は init なし）
    pub fn change_plan(ctx: Context<ChangePlan>, level: u8) -> Result<()> {
        let new_plan = match level {
            1 => 100,
            2 => 200,
            _ => 0,
        };
        if new_plan == 0 {
            return Ok(());
        }
        let hist = &mut ctx.accounts.history; // ← init なし（本来は初期化すべき）
        hist.plan = new_plan;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(init, payer = admin, space = 8 + 128 + 1)]
    pub user_account: Account<'info, UserData>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangePlan<'info> {
    pub user_account: Account<'info, UserData>, // ← init なし
    pub history: Account<'info, HistoryData>,   // ← init なし
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserData {
    pub email: String,
    pub plan: u32,
}

#[account]
pub struct HistoryData {
    pub plan: u32,
}
