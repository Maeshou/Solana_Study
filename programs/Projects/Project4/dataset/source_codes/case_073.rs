use anchor_lang::prelude::*;

declare_id!("SafeEx19Fuel1111111111111111111111111111111");

#[program]
pub mod example19 {
    use super::*;

    pub fn init_fuel(
        ctx: Context<InitFuel>,
        capacity: u32,
    ) -> Result<()> {
        let f = &mut ctx.accounts.fuel;
        f.tank_capacity = capacity;
        f.fuel_level    = capacity;
        f.reserve_flag  = false;

        // 初回チェック：5%以下ならリザーブ
        let threshold = capacity * 5 / 100;
        if f.fuel_level <= threshold {
            f.reserve_flag = true;
        }
        Ok(())
    }

    pub fn consume(
        ctx: Context<Consume>,
        amount: u32,
    ) -> Result<()> {
        let f = &mut ctx.accounts.fuel;
        // 大量消費時は2段階
        if amount > f.tank_capacity / 2 {
            f.fuel_level = f.fuel_level.saturating_sub(f.tank_capacity / 4);
        }
        f.fuel_level = f.fuel_level.saturating_sub(amount.min(f.fuel_level));

        // リザーブ判定
        let threshold = f.tank_capacity * 10 / 100;
        if f.fuel_level <= threshold {
            f.reserve_flag = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitFuel<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 1)]
    pub fuel: Account<'info, FuelData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Consume<'info> {
    #[account(mut)] pub fuel: Account<'info, FuelData>,
}

#[account]
pub struct FuelData {
    pub tank_capacity: u32,
    pub fuel_level:    u32,
    pub reserve_flag:  bool,
}
