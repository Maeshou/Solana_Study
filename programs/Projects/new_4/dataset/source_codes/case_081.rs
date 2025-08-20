use anchor_lang::prelude::*;

declare_id!("NextCaseEx60606060606060606060606060606060");

#[program]
pub mod example6 {
    use super::*;

    // 旅行を予約（reservation にだけ init）
    pub fn book_trip(ctx: Context<BookTrip>, nights: u8) -> Result<()> {
        let res = &mut ctx.accounts.reservation;       // ← initあり
        res.nights = nights;

        let cancel = &mut ctx.accounts.cancellation;   // ← initなし（本来は初期化すべき）
        // 宿泊日数が 0 なら自動キャンセル
        if nights == 0 {
            cancel.flag = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BookTrip<'info> {
    #[account(init, payer = user, space = 8 + 1)]
    pub reservation: Account<'info, ReservationData>,
    pub cancellation: Account<'info, CancellationData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ReservationData {
    pub nights: u8,
}

#[account]
pub struct CancellationData {
    pub flag: bool,
}
