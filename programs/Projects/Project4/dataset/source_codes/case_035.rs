use anchor_lang::prelude::*;

declare_id!("Repertory20Share111111111111111111111111111111");

#[program]
pub mod sharing {
    use super::*;

    // 予約を作成
    pub fn book_item(ctx: Context<BookItem>, item_id: u64, until: i64) -> Result<()> {
        let b = &mut ctx.accounts.booking;
        b.item_id = item_id;
        b.user = ctx.accounts.user.key();
        b.expires = until;
        b.reviewed = false;
        Ok(())
    }

    // レビューを追加
    pub fn add_review(ctx: Context<AddReview>, rating: u8, comment: String) -> Result<()> {
        let b = &mut ctx.accounts.booking;          // ← initなし：既存参照
        if !b.reviewed && rating > 0 {
            b.rating = rating;
            b.comment = comment;
            b.reviewed = true;
            b.reviewed_at = Clock::get()?.unix_timestamp;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BookItem<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32 + 8 + 1 + 1)]
    pub booking: Account<'info, BookingData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddReview<'info> {
    pub booking: Account<'info, BookingData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BookingData {
    pub item_id: u64,
    pub user: Pubkey,
    pub expires: i64,
    pub reviewed: bool,
    pub rating: u8,
    pub comment: String,
    pub reviewed_at: i64,
}
