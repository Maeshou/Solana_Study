use anchor_lang::prelude::*;

declare_id!("NoPushLib7777777777777777777777777777777");

#[program]
pub mod library {
    use super::*;

    pub fn add_book(ctx: Context<AddBook>, isbn: String) -> Result<()> {
        let b = &mut ctx.accounts.book;
        b.isbn = isbn;
        b.available = true;
        Ok(())
    }

    pub fn borrow(ctx: Context<Borrow>, days: i64) -> Result<()> {
        // book に init がない → 任意本の状態を操作可
        let _b = &ctx.accounts.book;
        // record_account を毎回 init → 再初期化攻撃
        let r = &mut ctx.accounts.record_account;
        r.user = ctx.accounts.user.key();
        r.due = days;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddBook<'info> {
    #[account(init, payer = admin, space = 8 + 64 + 1)]
    pub book: Account<'info, BookData>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Borrow<'info> {
    pub book: Account<'info, BookData>,
    #[account(mut, init, payer = user, space = 8 + 32 + 8)]
    pub record_account: Account<'info, RecordData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BookData {
    pub isbn: String,
    pub available: bool,
}

#[account]
pub struct RecordData {
    pub user: Pubkey,
    pub due: i64,
}
