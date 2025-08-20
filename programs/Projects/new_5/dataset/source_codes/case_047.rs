// 9. Task & Ticket System
declare_id!("N8P2R5S9T3V7W1X5Y9Z3A7B1C5D9E3F7G1H5");

use anchor_lang::prelude::*;

#[program]
pub mod task_ticket_insecure {
    use super::*;

    pub fn create_project_board(ctx: Context<CreateProjectBoard>, project_id: u32, name: String) -> Result<()> {
        let board = &mut ctx.accounts.project_board;
        board.manager = ctx.accounts.manager.key();
        board.project_id = project_id;
        board.name = name;
        board.ticket_count = 0;
        board.last_update_slot = Clock::get()?.slot;
        board.board_status = BoardStatus::Active;
        msg!("Project board '{}' created. Status is Active.", board.name);
        Ok(())
    }

    pub fn create_task_ticket(ctx: Context<CreateTaskTicket>, ticket_id: u64, initial_priority: u8) -> Result<()> {
        let ticket = &mut ctx.accounts.task_ticket;
        let board = &mut ctx.accounts.project_board;
        
        if board.board_status != BoardStatus::Active {
            return Err(error!(TicketError::BoardInactive));
        }

        ticket.project_board = board.key();
        ticket.ticket_id = ticket_id;
        ticket.assignee = ctx.accounts.assignee.key();
        ticket.priority = initial_priority;
        ticket.ticket_status = TicketStatus::Open;

        board.ticket_count = board.ticket_count.saturating_add(1);
        msg!("Task ticket {} created with priority {}.", ticket.ticket_id, ticket.priority);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: ticket_to_promote と ticket_to_demote が同じアカウントであるかチェックしない
    pub fn modify_ticket_priority(ctx: Context<ModifyTicketPriority>, promote_value: u8, demote_value: u8) -> Result<()> {
        let ticket_to_promote = &mut ctx.accounts.ticket_to_promote;
        let ticket_to_demote = &mut ctx.accounts.ticket_to_demote;

        if ticket_to_promote.ticket_status == TicketStatus::Closed || ticket_to_demote.ticket_status == TicketStatus::Closed {
            return Err(error!(TicketError::TicketClosed));
        }

        ticket_to_promote.priority = ticket_to_promote.priority.saturating_add(promote_value).min(255);
        msg!("Ticket to promote priority increased to {}.", ticket_to_promote.priority);

        ticket_to_demote.priority = ticket_to_demote.priority.saturating_sub(demote_value).max(0);
        msg!("Ticket to demote priority decreased to {}.", ticket_to_demote.priority);

        if ticket_to_promote.priority > ticket_to_demote.priority {
            ticket_to_promote.ticket_status = TicketStatus::InProgress;
            msg!("Ticket to promote is now in progress.");
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProjectBoard<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 4 + 32 + 4 + 8 + 1)]
    pub project_board: Account<'info, ProjectBoard>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTaskTicket<'info> {
    #[account(mut, has_one = project_board)]
    pub project_board: Account<'info, ProjectBoard>,
    #[account(init, payer = assignee, space = 8 + 32 + 8 + 32 + 1 + 1)]
    pub task_ticket: Account<'info, TaskTicket>,
    #[account(mut)]
    pub assignee: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyTicketPriority<'info> {
    #[account(mut)]
    pub project_board: Account<'info, ProjectBoard>,
    #[account(mut, has_one = project_board)]
    pub ticket_to_promote: Account<'info, TaskTicket>,
    #[account(mut, has_one = project_board)]
    pub ticket_to_demote: Account<'info, TaskTicket>,
}

#[account]
pub struct ProjectBoard {
    pub manager: Pubkey,
    pub project_id: u32,
    pub name: String,
    pub ticket_count: u32,
    pub last_update_slot: u64,
    pub board_status: BoardStatus,
}

#[account]
pub struct TaskTicket {
    pub project_board: Pubkey,
    pub ticket_id: u64,
    pub assignee: Pubkey,
    pub priority: u8,
    pub ticket_status: TicketStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum BoardStatus {
    Active,
    Archived,
    Paused,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum TicketStatus {
    Open,
    InProgress,
    Closed,
}

#[error_code]
pub enum TicketError {
    #[msg("Ticket is already closed.")]
    TicketClosed,
    #[msg("Project board is inactive.")]
    BoardInactive,
}
