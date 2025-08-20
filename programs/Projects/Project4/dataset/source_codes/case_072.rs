use anchor_lang::prelude::*;

declare_id!("SafeEx18Battery111111111111111111111111111");

#[program]
pub mod example18 {
    use super::*;

    pub fn init_battery(
        ctx: Context<InitBattery>,
        cycles: u32,
    ) -> Result<()> {
        let b = &mut ctx.accounts.battery;
        b.charge_level = 100;
        b.cycles       = cycles;
        b.health_flag  = false;

        // サイクル数に応じて劣化率設定
        let mut degrade = 0u32;
        if cycles > 1000 {
            degrade = 20;
        } else {
            degrade = 5;
        }
        // 劣化後レベル
        b.charge_level = b.charge_level.saturating_sub(degrade);
        // 健康フラグ
        if degrade > 10 {
            b.health_flag = true;
        }
        Ok(())
    }

    pub fn discharge(
        ctx: Context<Discharge>,
        amount: u32,
    ) -> Result<()> {
        let b = &mut ctx.accounts.battery;
        // 二段階減少
        if amount > 50 {
            b.charge_level = b.charge_level.saturating_sub(30);
        }
        b.charge_level = b.charge_level.saturating_sub(amount.min(b.charge_level));
        // フラグ更新
        if b.charge_level < 20 {
            b.health_flag = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBattery<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 1)]
    pub battery: Account<'info, BatteryData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Discharge<'info> {
    #[account(mut)] pub battery: Account<'info, BatteryData>,
}

#[account]
pub struct BatteryData {
    pub charge_level: u32,
    pub cycles:       u32,
    pub health_flag:  bool,
}
