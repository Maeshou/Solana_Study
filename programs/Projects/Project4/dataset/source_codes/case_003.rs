use anchor_lang::prelude::*;

declare_id!("NoPushSub44444444444444444444444444444444");

#[program]
pub mod subscr {
    use super::*;

    pub fn register(ctx: Context<Register>, email: String) -> Result<()> {
        let u = &mut ctx.accounts.user;
        u.email = email;
        u.plan = 1;
        Ok(())
    }

    pub fn change_plan(ctx: Context<Change>, new_tier: u8) -> Result<()> {
        // user に init がない → 別ユーザーのプラン変更可
        let u = &mut ctx.accounts.user;
        u.plan = new_tier;
        // history_account を毎回 init → 再初期化攻撃可
        let h = &mut ctx.accounts.history;
        h.tier = new_tier;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(init, payer = admin, space = 8 + 128 + 1)]
    pub user: Account<'info, UserData>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Change<'info> {
    pub user: Account<'info, UserData>,
    #[account(mut, init, payer = admin, space = 8 + 1)]
    pub history: Account<'info, HistoryData>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserData {
    pub email: String,
    pub plan: u8,
}

#[account]
pub struct HistoryData {
    pub tier: u8,
}
