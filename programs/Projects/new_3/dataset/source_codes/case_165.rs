use anchor_lang::prelude::*;
declare_id!("HotelResvXyz1111111111111111111111111111111");

/// ホテル情報
#[account]
pub struct Hotel {
    pub manager:             Pubkey, // ホテル管理者
    pub total_reservations:  u64,    // これまでの予約件数
}

/// 予約情報
#[account]
pub struct Reservation {
    pub guest:    Pubkey, // 予約したお客様
    pub hotel:    Pubkey, // 本来は Hotel.key() と一致すべき
    pub nights:   u8,     // 宿泊日数
}

#[derive(Accounts)]
pub struct BookRoom<'info> {
    /// Hotel.manager == manager.key() は検証される
    #[account(mut, has_one = manager)]
    pub hotel:        Account<'info, Hotel>,

    /// Reservation.hotel ⇔ hotel.key() の検証がないまま初期化
    #[account(init, payer = guest, space = 8 + 32 + 32 + 1)]
    pub reservation:  Account<'info, Reservation>,

    #[account(mut)]
    pub manager:      Signer<'info>,
    #[account(mut)]
    pub guest:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmStay<'info> {
    /// Reservation.guest == guest.key() は検証される
    #[account(mut, has_one = guest)]
    pub reservation:  Account<'info, Reservation>,

    /// hotel.account と reservation.hotel の一致チェックがない
    #[account(mut)]
    pub hotel:        Account<'info, Hotel>,

    pub guest:        Signer<'info>,
}

#[program]
pub mod hotel_reservation_vuln {
    use super::*;

    /// ルームを予約
    pub fn book_room(
        ctx: Context<BookRoom>,
        nights: u8
    ) -> Result<()> {
        let h = &mut ctx.accounts.hotel;
        let r = &mut ctx.accounts.reservation;
        // 脆弱性ポイント:
        // r.hotel = h.key(); と代入するだけで、
        // Reservation.hotel と Hotel.key() の照合を行っていない
        r.guest  = ctx.accounts.guest.key();
        r.hotel  = h.key();
        r.nights = nights;

        // 予約件数をインクリメント
        h.total_reservations += 1;
        Ok(())
    }

    /// 宿泊を確定
    pub fn confirm_stay(ctx: Context<ConfirmStay>) -> Result<()> {
        let h = &mut ctx.accounts.hotel;
        // 本来は必須:
        // require_keys_eq!(
        //     ctx.accounts.reservation.hotel,
        //     h.key(),
        //     ErrorCode::HotelMismatch
        // );
        // がないため、攻撃者は別ホテルの Reservation を渡し、
        // 予約件数を不正に減らせてしまう

        // 予約件数をデクリメント
        h.total_reservations -= 1;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Reservation が指定の Hotel と一致しません")]
    HotelMismatch,
}
