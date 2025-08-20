use anchor_lang::prelude::*;

declare_id!("Ex1000000000000000000000000000000000010");

#[program]
pub mod example10 {
    use super::*;

    // 車両を登録し、次回整備予定日を計算
    pub fn register_vehicle(ctx: Context<RegVeh>, vin: String) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let v = &mut ctx.accounts.vehicle;          // ← initあり
        v.vin = vin;
        v.maint_count = 0;
        v.registered_at = now;
        // 30日後を秒で計算
        v.next_service = now + 30 * 24 * 60 * 60;
        Ok(())
    }

    // 整備を記録し、整備回数と次回予定を更新
    pub fn service_vehicle(ctx: Context<ServiceVeh>) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let v = &mut ctx.accounts.vehicle;          // ← initなし：既存参照のみ
        v.maint_count += 1;
        v.last_service = now;
        v.next_service = now + 30 * 24 * 60 * 60;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegVeh<'info> {
    #[account(init, payer = owner, space = 8 + 64 + 4 + 8*4)]
    pub vehicle: Account<'info, VehicleData>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ServiceVeh<'info> {
    pub vehicle: Account<'info, VehicleData>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VehicleData {
    pub vin: String,
    pub maint_count: u32,
    pub registered_at: i64,
    pub last_service: i64,
    pub next_service: i64,
}
