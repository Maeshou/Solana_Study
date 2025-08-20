use anchor_lang::prelude::*;

declare_id!("SpinWhel1010101010101010101010101010101010");

#[program]
pub mod spin_wheel {
    use super::*;

    pub fn init_wheel(ctx: Context<InitWheel>) -> Result<()> {
        // 初期状態は空の履歴
        Ok(())
    }

    pub fn spin(ctx: Context<ModifyWheel>, reward: u64) -> Result<()> {
        let w = &mut ctx.accounts.wheel;
        w.history.push(reward);
        w.spin_count = w.spin_count.saturating_add(1);
        // 履歴は最新5件のみ保持
        if w.history.len() as u8 > 5 {
            w.history.remove(0);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWheel<'info> {
    #[account(init, payer = user, space = 8 + 8 + 4 + (8 * 5))]
    pub wheel: Account<'info, WheelData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyWheel<'info> {
    #[account(mut)]
    pub wheel: Account<'info, WheelData>,
    pub user: Signer<'info>,
}

#[account]
pub struct WheelData {
    pub spin_count: u64,
    pub history: Vec<u64>,
}
