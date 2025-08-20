// 4. Library Management System
declare_id!("L7I3B8R5A9R2Y6M4A7N1A5G9E3M7E1N5T");

use anchor_lang::prelude::*;

#[program]
pub mod library_insecure {
    use super::*;

    pub fn create_library(ctx: Context<CreateLibrary>, library_id: u64, max_books: u32) -> Result<()> {
        let library = &mut ctx.accounts.library;
        library.manager = ctx.accounts.manager.key();
        library.library_id = library_id;
        library.total_books = 0;
        library.max_books = max_books;
        library.library_status = LibraryStatus::Open;
        msg!("Library {} created with capacity {}.", library.library_id, library.max_books);
        Ok(())
    }

    pub fn check_out_book(ctx: Context<CheckOutBook>, book_id: u32, user_id: u32) -> Result<()> {
        let book = &mut ctx.accounts.book;
        let library = &mut ctx.accounts.library;
        
        if matches!(library.library_status, LibraryStatus::Open) {
            if book.is_available {
                book.is_available = false;
                book.checked_out_by = user_id;
                msg!("Book {} checked out by user {}.", book.book_id, book.checked_out_by);
            } else {
                msg!("Book {} is not available for checkout.", book.book_id);
            }
        } else {
            msg!("Library is closed. Cannot check out books.");
        }

        book.library = library.key();
        book.book_id = book_id;
        Ok(())
    }

    pub fn return_book(ctx: Context<ReturnBook>, user_id: u32) -> Result<()> {
        let book1 = &mut ctx.accounts.book1;
        let book2 = &mut ctx.accounts.book2;
        
        if book1.checked_out_by == user_id && book2.checked_out_by != user_id {
            book1.is_available = true;
            book1.checked_out_by = 0;
            msg!("Book 1 returned.");
        } else {
            msg!("Book 1 was not checked out by this user.");
        }

        if book2.checked_out_by == user_id {
            book2.is_available = true;
            book2.checked_out_by = 0;
            msg!("Book 2 returned.");
        } else {
            msg!("Book 2 was not checked out by this user.");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateLibrary<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 4 + 4 + 1)]
    pub library: Account<'info, Library>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CheckOutBook<'info> {
    #[account(mut)]
    pub library: Account<'info, Library>,
    #[account(init, payer = borrower, space = 8 + 32 + 4 + 4 + 1 + 1)]
    pub book: Account<'info, Book>,
    #[account(mut)]
    pub borrower: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReturnBook<'info> {
    #[account(mut, has_one = library)]
    pub library: Account<'info, Library>,
    #[account(mut, has_one = library)]
    pub book1: Account<'info, Book>,
    #[account(mut, has_one = library)]
    pub book2: Account<'info, Book>,
}

#[account]
pub struct Library {
    pub manager: Pubkey,
    pub library_id: u64,
    pub total_books: u32,
    pub max_books: u32,
    pub library_status: LibraryStatus,
}

#[account]
pub struct Book {
    pub library: Pubkey,
    pub book_id: u32,
    pub checked_out_by: u32,
    pub is_available: bool,
    pub book_type: BookType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum LibraryStatus {
    Open,
    Closed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum BookType {
    Fiction,
    NonFiction,
}