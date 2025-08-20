// 10. Financial Ledger & Transaction Processing
declare_id!("F3G6H0J3K7L1M5N9P3Q7R1S5T9U3V7W1X5Y9");

use anchor_lang::prelude::*;

#[program]
pub mod financial_ledger_insecure {
    use super::*;

    pub fn setup_ledger_book(ctx: Context<SetupLedgerBook>, book_id: u64, name: String) -> Result<()> {
        let book = &mut ctx.accounts.ledger_book;
        book.owner = ctx.accounts.owner.key();
        book.book_id = book_id;
        book.name = name;
        book.transaction_count = 0;
        book.total_volume = 1000;
        book.ledger_status = LedgerStatus::Open;
        msg!("Financial Ledger Book '{}' created with initial volume of 1000. Status is Open.", book.name);
        Ok(())
    }

    pub fn record_transaction(ctx: Context<RecordTransaction>, transaction_id: u64, amount: u64) -> Result<()> {
        let transaction_record = &mut ctx.accounts.transaction_record;
        let book = &mut ctx.accounts.ledger_book;
        
        if book.ledger_status != LedgerStatus::Open {
            return Err(error!(FinancialError::LedgerClosed));
        }

        transaction_record.ledger_book = book.key();
        transaction_record.transaction_id = transaction_id;
        transaction_record.sender = ctx.accounts.sender.key();
        transaction_record.receiver = ctx.accounts.receiver.key();
        transaction_record.amount = amount;
        transaction_record.processing_status = ProcessingStatus::Pending;

        book.transaction_count = book.transaction_count.saturating_add(1);
        book.total_volume = book.total_volume.saturating_add(amount);
        msg!("Transaction record {} logged with amount {}.", transaction_record.transaction_id, transaction_record.amount);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: first_record と second_record が同じアカウントであるかチェックしない
    pub fn process_multiple_transactions(ctx: Context<ProcessMultipleTransactions>) -> Result<()> {
        let first_record = &mut ctx.accounts.first_record;
        let second_record = &mut ctx.accounts.second_record;
        
        if first_record.processing_status != ProcessingStatus::Pending || second_record.processing_status != ProcessingStatus::Pending {
            return Err(error!(FinancialError::TransactionNotPending));
        }

        let mut loop_count = 0;
        while loop_count < 2 {
            if first_record.amount > second_record.amount {
                first_record.amount = first_record.amount.saturating_sub(10);
                second_record.amount = second_record.amount.saturating_add(20);
                msg!("First record has larger amount, adjusting.");
            } else {
                first_record.amount = first_record.amount.saturating_add(20);
                second_record.amount = second_record.amount.saturating_sub(10);
                msg!("Second record has larger or equal amount, adjusting.");
            }
            loop_count += 1;
        }

        if first_record.amount > 1000 {
            first_record.processing_status = ProcessingStatus::Processed;
            msg!("First record processed due to large amount.");
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupLedgerBook<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 32 + 4 + 8 + 1)]
    pub ledger_book: Account<'info, LedgerBook>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordTransaction<'info> {
    #[account(mut, has_one = ledger_book)]
    pub ledger_book: Account<'info, LedgerBook>,
    #[account(init, payer = sender, space = 8 + 32 + 8 + 32 + 32 + 8 + 1)]
    pub transaction_record: Account<'info, TransactionRecord>,
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK: This is the receiver of the transaction, not a signer.
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessMultipleTransactions<'info> {
    #[account(mut)]
    pub ledger_book: Account<'info, LedgerBook>,
    #[account(mut, has_one = ledger_book)]
    pub first_record: Account<'info, TransactionRecord>,
    #[account(mut, has_one = ledger_book)]
    pub second_record: Account<'info, TransactionRecord>,
}

#[account]
pub struct LedgerBook {
    pub owner: Pubkey,
    pub book_id: u64,
    pub name: String,
    pub transaction_count: u32,
    pub total_volume: u64,
    pub ledger_status: LedgerStatus,
}

#[account]
pub struct TransactionRecord {
    pub ledger_book: Pubkey,
    pub transaction_id: u64,
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub amount: u64,
    pub processing_status: ProcessingStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum LedgerStatus {
    Open,
    Closed,
    Archived,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ProcessingStatus {
    Pending,
    Processed,
    Failed,
}

#[error_code]
pub enum FinancialError {
    #[msg("Ledger is closed.")]
    LedgerClosed,
    #[msg("Transaction is not in a pending state.")]
    TransactionNotPending,
}