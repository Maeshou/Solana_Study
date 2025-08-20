// 8. Event Log & Participation Records
declare_id!("Q4R7T1U5V9W3X7Y2Z6A0B4C8D2E6F0G5H9I3");

use anchor_lang::prelude::*;

#[program]
pub mod event_log_insecure {
    use super::*;

    pub fn create_event_ledger(ctx: Context<CreateEventLedger>, event_id: u32, event_type: String) -> Result<()> {
        let ledger = &mut ctx.accounts.event_ledger;
        ledger.host = ctx.accounts.host.key();
        ledger.event_id = event_id;
        ledger.event_type = event_type;
        ledger.participant_count = 0;
        ledger.last_entry_id = 0; // Counter for demonstration
        ledger.event_status = EventStatus::Active;
        msg!("Event ledger for event {} created. Status is Active.", ledger.event_id);
        Ok(())
    }

    pub fn log_participation_record(ctx: Context<LogParticipationRecord>, participant_id: u64, is_vip: bool) -> Result<()> {
        let record = &mut ctx.accounts.participation_record;
        let ledger = &mut ctx.accounts.event_ledger;
        
        if ledger.event_status != EventStatus::Active {
            return Err(error!(EventError::LedgerInactive));
        }

        record.event_ledger = ledger.key();
        record.participant_id = participant_id;
        record.participant = ctx.accounts.participant.key();
        record.is_vip = is_vip;
        record.attendance_status = AttendanceStatus::Registered;
        
        ledger.participant_count = ledger.participant_count.saturating_add(1);
        msg!("Participation record for participant {} logged. Attendance status: Registered.", record.participant_id);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: entry_one と entry_two が同じアカウントであるかチェックしない
    pub fn update_attendance_status(ctx: Context<UpdateAttendanceStatus>, attendance_scores: Vec<u8>) -> Result<()> {
        let entry_one = &mut ctx.accounts.entry_one;
        let entry_two = &mut ctx.accounts.entry_two;
        
        if entry_one.attendance_status != AttendanceStatus::Registered || entry_two.attendance_status != AttendanceStatus::Registered {
            return Err(error!(EventError::EntryNotRegistered));
        }

        let mut average_score = 0;
        if !attendance_scores.is_empty() {
            average_score = attendance_scores.iter().sum::<u8>() / attendance_scores.len() as u8;
        }

        if average_score > 50 {
            entry_one.attendance_status = AttendanceStatus::Attended;
            entry_two.attendance_status = AttendanceStatus::Attended;
            msg!("Both entries marked as attended due to high average score.");
        } else {
            entry_one.attendance_status = AttendanceStatus::DidNotAttend;
            entry_two.attendance_status = AttendanceStatus::DidNotAttend;
            msg!("Both entries marked as did not attend due to low average score.");
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEventLedger<'info> {
    #[account(init, payer = host, space = 8 + 32 + 4 + 32 + 4 + 8 + 1)]
    pub event_ledger: Account<'info, EventLedger>,
    #[account(mut)]
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LogParticipationRecord<'info> {
    #[account(mut, has_one = event_ledger)]
    pub event_ledger: Account<'info, EventLedger>,
    #[account(init, payer = participant, space = 8 + 32 + 8 + 32 + 1 + 1)]
    pub participation_record: Account<'info, ParticipationRecord>,
    #[account(mut)]
    pub participant: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAttendanceStatus<'info> {
    #[account(mut)]
    pub event_ledger: Account<'info, EventLedger>,
    #[account(mut, has_one = event_ledger)]
    pub entry_one: Account<'info, ParticipationRecord>,
    #[account(mut, has_one = event_ledger)]
    pub entry_two: Account<'info, ParticipationRecord>,
}

#[account]
pub struct EventLedger {
    pub host: Pubkey,
    pub event_id: u32,
    pub event_type: String,
    pub participant_count: u32,
    pub last_entry_id: u64,
    pub event_status: EventStatus,
}

#[account]
pub struct ParticipationRecord {
    pub event_ledger: Pubkey,
    pub participant_id: u64,
    pub participant: Pubkey,
    pub is_vip: bool,
    pub attendance_status: AttendanceStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum EventStatus {
    Active,
    Archived,
    Canceled,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum AttendanceStatus {
    Registered,
    Attended,
    DidNotAttend,
}

#[error_code]
pub enum EventError {
    #[msg("Event ledger is inactive.")]
    LedgerInactive,
    #[msg("Participation record is not in a registered state.")]
    EntryNotRegistered,
}
