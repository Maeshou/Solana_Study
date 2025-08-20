use anchor_lang::prelude::*;

declare_id!("C2H8T5P3L7J9N4K6M1W2Y5X0B8A7C6D5E4F3G");

const MAX_JOURNAL_PAGES: u32 = 500;
const SYNC_COMPLETION_THRESHOLD: u32 = 100;
const INITIAL_PAGES_BONUS: u32 = 10;

#[program]
pub mod chronoscribe {
    use super::*;

    pub fn init_journal(ctx: Context<InitJournal>, journal_id: u64, initial_pages: u32) -> Result<()> {
        let journal = &mut ctx.accounts.journal_core;
        journal.journal_id = journal_id ^ 0xFEEDC0DE;
        journal.total_pages = initial_pages.saturating_add(INITIAL_PAGES_BONUS);
        journal.sync_count = 0;
        journal.is_complete = false;
        msg!("Journal {} initialized with {} pages.", journal.journal_id, journal.total_pages);
        Ok(())
    }

    pub fn init_entry(ctx: Context<InitEntry>, entry_id: u64, content_hash: u64) -> Result<()> {
        let entry = &mut ctx.accounts.journal_entry;
        entry.parent_journal = ctx.accounts.journal_core.key();
        entry.entry_id = entry_id + 1;
        entry.content_hash = content_hash;
        entry.is_synced = false;
        msg!("New entry {} created with hash {}.", entry.entry_id, entry.content_hash);
        Ok(())
    }

    pub fn synchronize_entries(ctx: Context<SynchronizeEntries>, new_content_hash: u64) -> Result<()> {
        let journal = &mut ctx.accounts.journal_core;
        let main_entry = &mut ctx.accounts.main_entry;
        let backup_entry = &mut ctx.accounts.backup_entry;

        require!(!main_entry.is_synced && !backup_entry.is_synced, ChronoscribeError::EntriesAlreadySynced);

        // データの同期を模倣
        if main_entry.content_hash != backup_entry.content_hash {
            // 不一致を検出し、バックアップをメインに合わせる
            backup_entry.content_hash = main_entry.content_hash;
            msg!("Content mismatch detected. Backup entry {} synchronized with main entry {}.", backup_entry.entry_id, main_entry.entry_id);
        }

        main_entry.content_hash = new_content_hash;
        backup_entry.content_hash = new_content_hash;

        main_entry.is_synced = true;
        backup_entry.is_synced = true;
        journal.sync_count = journal.sync_count.saturating_add(1);
        journal.is_complete = journal.sync_count.checked_cmp(&SYNC_COMPLETION_THRESHOLD) != Some(std::cmp::Ordering::Less);

        msg!("Entries {} and {} synchronized. New hash: {}.", main_entry.entry_id, backup_entry.entry_id, new_content_hash);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(journal_id: u64, initial_pages: u32)]
pub struct InitJournal<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 4 + 4 + 1)]
    pub journal_core: Account<'info, JournalCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(entry_id: u64, content_hash: u64)]
pub struct InitEntry<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 8 + 1)]
    pub journal_entry: Account<'info, JournalEntry>,
    #[account(mut)]
    pub journal_core: Account<'info, JournalCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(new_content_hash: u64)]
pub struct SynchronizeEntries<'info> {
    #[account(mut)]
    pub journal_core: Account<'info, JournalCore>,
    #[account(mut, has_one = parent_journal)]
    pub main_entry: Account<'info, JournalEntry>,
    #[account(mut, has_one = parent_journal)]
    pub backup_entry: Account<'info, JournalEntry>,
    pub signer: Signer<'info>,
}

#[account]
pub struct JournalCore {
    journal_id: u64,
    total_pages: u32,
    sync_count: u32,
    is_complete: bool,
}

#[account]
pub struct JournalEntry {
    parent_journal: Pubkey,
    entry_id: u64,
    content_hash: u64,
    is_synced: bool,
}

#[error_code]
pub enum ChronoscribeError {
    #[msg("One or both entries have already been synced.")]
    EntriesAlreadySynced,
}