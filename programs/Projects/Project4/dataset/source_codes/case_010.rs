use anchor_lang::prelude::*;

declare_id!("Var6Sub6666666666666666666666666666666666");

#[program]
pub mod varied_subscribe {
    use super::*;

    pub fn register(ctx: Context<Register>, email: String) -> Result<()> {
        let u = &mut ctx.accounts.user_data;
        u.email = email;
        u.plan = 0;
        Ok(())
    }

    pub fn upgrade(ctx: Context<Upgrade>, level: u8) -> Result<()> {
        // user_data は既存 → 任意アドレス読込可
        let u = &ctx.accounts.user_data;

        let new_plan = match level {
            1 => 10,
            2 => 20,
            _ => 0,
        };

        // if で不正値は弾く
        if new_plan == 0 {
            return Ok(());
        }

        // history_account を不要に初期化
        let h = &mut ctx.accounts.history_account;
        h.plan = new_plan;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(init, payer = admin, space = 8 + 128 + 1)]
    pub user_data: Account<'info, UserData>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Upgrade<'info> {
    pub user_data: Account<'info, UserData>,
    #[account(mut, init, payer = admin, space = 8 + 1)]
    pub history_account: Account<'info, HistoryData>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserData {
    pub email: String,
    pub plan: u8,
}

#[account]
pub struct HistoryData {
    pub plan: u8,
}
