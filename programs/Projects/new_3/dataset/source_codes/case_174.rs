use anchor_lang::prelude::*;
declare_id!("VehRentVuln111111111111111111111111111111");

/// 車両情報
#[account]
pub struct Vehicle {
    pub owner:       Pubkey,  // 車両所有者
    pub model:       String,  // 車種
    pub mileage:     u64,     // 走行距離
}

/// レンタル予約
#[account]
pub struct Booking {
    pub renter:      Pubkey,  // 予約者
    pub vehicle:     Pubkey,  // 本来は Vehicle.key() と一致すべき
    pub notes:       String,  // 連絡メモ
}

#[derive(Accounts)]
pub struct RegisterVehicle<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 64 + 8)]
    pub vehicle:     Account<'info, Vehicle>,
    #[account(mut)]
    pub owner:       Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MakeBooking<'info> {
    /// Vehicle.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub vehicle:     Account<'info, Vehicle>,

    /// Booking.vehicle ⇔ vehicle.key() の検証がないため、
    /// 任意の Booking.account を渡しても通ってしまう
    #[account(init, payer = renter, space = 8 + 32 + 32 + 4 + 128)]
    pub booking:     Account<'info, Booking>,

    #[account(mut)]
    pub owner:       Signer<'info>,
    #[account(mut)]
    pub renter:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelBooking<'info> {
    /// Booking.renter == renter.key() は検証される
    #[account(mut, has_one = renter)]
    pub booking:     Account<'info, Booking>,

    /// Vehicle.account と Booking.vehicle の照合がない
    #[account(mut)]
    pub vehicle:     Account<'info, Vehicle>,

    pub renter:      Signer<'info>,
}

#[program]
pub mod veh_rent_vuln {
    use super::*;

    /// 車両を登録
    pub fn register_vehicle(ctx: Context<RegisterVehicle>, model: String, mileage: u64) -> Result<()> {
        let v = &mut ctx.accounts.vehicle;
        v.owner   = ctx.accounts.owner.key();
        v.model   = model;
        v.mileage = mileage;
        Ok(())
    }

    /// レンタル予約を作成
    pub fn make_booking(ctx: Context<MakeBooking>, notes: String) -> Result<()> {
        let b = &mut ctx.accounts.booking;
        // 脆弱性ポイント:
        // b.vehicle = ctx.accounts.vehicle.key(); と設定するのみで、
        // Booking.vehicle と Vehicle.key() の一致検証がない
        b.renter  = ctx.accounts.renter.key();
        b.vehicle = ctx.accounts.vehicle.key();
        b.notes   = notes;
        Ok(())
    }

    /// レンタル予約をキャンセル
    pub fn cancel_booking(ctx: Context<CancelBooking>) -> Result<()> {
        let _v = &mut ctx.accounts.vehicle;
        let b  = &mut ctx.accounts.booking;
        // 本来は必須:
        // require_keys_eq!(b.vehicle, ctx.accounts.vehicle.key(), ErrorCode::BookingMismatch);
        // がないため、攻撃者は自分の Booking で任意の車両予約をキャンセルできる
        b.notes = String::from("Cancelled by renter");
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Booking が指定の Vehicle と一致しません")]
    BookingMismatch,
}
