// 8. Event Log & Participation Records
declare_id!("Z1A5B9C3D7E1F5G9H3I7J1K5L9M3N7P1Q5R9");

use anchor_lang::prelude::*;

#[program]
pub mod event_log_insecure {
    use super::*;

    pub fn init_event(ctx: Context<InitEvent>, event_id: u32, name: String) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.host = ctx.accounts.host.key();
        event.event_id = event_id;
        event.name = name;
        event.participant_count = 0;
        event.is_open = true;
        msg!("Event '{}' initialized.", event.name);
        Ok(())
    }

    pub fn init_participation(ctx: Context<InitParticipation>, participation_id: u64) -> Result<()> {
        let participation = &mut ctx.accounts.participation;
        let event = &mut ctx.accounts.event;
        
        participation.event = event.key();
        participation.participant = ctx.accounts.participant.key();
        participation.participation_id = participation_id;
        participation.has_attended = false;
        
        event.participant_count = event.participant_count.saturating_add(1);
        msg!("Participation {} created for event {}.", participation.participation_id, event.name);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: record_a と record_b が同じアカウントであるかチェックしない
    pub fn update_participation_records(ctx: Context<UpdateParticipationRecords>, update_values: Vec<u8>) -> Result<()> {
        let record_a = &mut ctx.accounts.record_a;
        let record_b = &mut ctx.accounts.record_b;

        if !ctx.accounts.event.is_open {
            return Err(ErrorCode::EventClosed.into());
        }

        let mut a_update_count = 0;
        let mut b_update_count = 0;

        for value in update_values.iter() {
            if *value > 5 {
                record_a.has_attended = true;
                record_b.has_attended = false;
                a_update_count += 1;
                msg!("Value > 5, updating A and B differently.");
            } else {
                record_a.has_attended = false;
                record_b.has_attended = true;
                b_update_count += 1;
                msg!("Value <= 5, updating B and A differently.");
            }
        }
        
        if record_a.has_attended {
            msg!("Record A is set to true.");
        } else {
            msg!("Record A is set to false.");
        }
        
        if record_b.has_attended {
            msg!("Record B is set to true.");
        } else {
            msg!("Record B is set to false.");
        }
        
        msg!("Processed {} updates. A updates: {}, B updates: {}.", update_values.len(), a_update_count, b_update_count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEvent<'info> {
    #[account(init, payer = host, space = 8 + 32 + 4 + 32 + 4 + 1)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitParticipation<'info> {
    #[account(mut, has_one = event)]
    pub event: Account<'info, Event>,
    #[account(init, payer = participant, space = 8 + 32 + 32 + 8 + 1)]
    pub participation: Account<'info, EventParticipation>,
    #[account(mut)]
    pub participant: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateParticipationRecords<'info> {
    #[account(mut)]
    pub event: Account<'info, Event>,
    #[account(mut, has_one = event)]
    pub record_a: Account<'info, EventParticipation>,
    #[account(mut, has_one = event)]
    pub record_b: Account<'info, EventParticipation>,
}

#[account]
pub struct Event {
    pub host: Pubkey,
    pub event_id: u32,
    pub name: String,
    pub participant_count: u32,
    pub is_open: bool,
}

#[account]
pub struct EventParticipation {
    pub event: Pubkey,
    pub participant: Pubkey,
    pub participation_id: u64,
    pub has_attended: bool,
}

#[error_code]
pub enum EventError {
    #[msg("Event is closed.")]
    EventClosed,
}
