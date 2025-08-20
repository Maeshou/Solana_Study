// 10. Task & Ticket System
declare_id!("N8P2R5S9T3V7W1X5Y9Z3A7B1C5D9E3F7G1H5");

use anchor_lang::prelude::*;

#[program]
pub mod task_ticket_insecure {
    use super::*;

    pub fn init_project(ctx: Context<InitProject>, project_id: u32, name: String) -> Result<()> {
        let project = &mut ctx.accounts.project;
        project.manager = ctx.accounts.manager.key();
        project.project_id = project_id;
        project.name = name;
        project.ticket_count = 0;
        project.is_active = true;
        msg!("Project '{}' initialized.", project.name);
        Ok(())
    }

    pub fn init_ticket(ctx: Context<InitTicket>, ticket_id: u64, initial_priority: u8) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket;
        let project = &mut ctx.accounts.project;
        
        ticket.project = project.key();
        ticket.ticket_id = ticket_id;
        ticket.reporter = ctx.accounts.reporter.key();
        ticket.priority = initial_priority;
        ticket.status = TicketStatus::Open;
        
        project.ticket_count = project.ticket_count.saturating_add(1);
        msg!("Ticket {} created for project {}.", ticket.ticket_id, project.name);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: ticket_a と ticket_b が同じアカウントであるかチェックしない
    pub fn assign_tickets(ctx: Context<AssignTickets>, assignees: Vec<Pubkey>) -> Result<()> {
        let ticket_a = &mut ctx.accounts.ticket_a;
        let ticket_b = &mut ctx.accounts.ticket_b;

        let mut assignee_a_found = false;
        let mut assignee_b_found = false;

        let mut loop_count = 0;
        for assignee in assignees.iter() {
            if loop_count >= 2 {
                break;
            }

            if ticket_a.status == TicketStatus::Open {
                if ticket_a.reporter == *assignee {
                    ticket_a.status = TicketStatus::Assigned;
                    msg!("Assigning A to reporter.");
                } else {
                    ticket_a.status = TicketStatus::InProgress;
                    msg!("Assigning A to non-reporter.");
                }
                assignee_a_found = true;
            }

            if ticket_b.status == TicketStatus::Open {
                if ticket_b.reporter == *assignee {
                    ticket_b.status = TicketStatus::Assigned;
                    msg!("Assigning B to reporter.");
                } else {
                    ticket_b.status = TicketStatus::InProgress;
                    msg!("Assigning B to non-reporter.");
                }
                assignee_b_found = true;
            }

            loop_count += 1;
        }

        if assignee_a_found {
            ticket_a.priority = ticket_a.priority.saturating_add(10).min(255);
            msg!("Ticket A priority increased to {}.", ticket_a.priority);
        } else {
            ticket_a.priority = ticket_a.priority.saturating_sub(5).max(0);
            msg!("Ticket A not assigned, priority decreased to {}.", ticket_a.priority);
        }

        if assignee_b_found {
            ticket_b.priority = ticket_b.priority.saturating_add(10).min(255);
            msg!("Ticket B priority increased to {}.", ticket_b.priority);
        } else {
            ticket_b.priority = ticket_b.priority.saturating_sub(5).max(0);
            msg!("Ticket B not assigned, priority decreased to {}.", ticket_b.priority);
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitProject<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 4 + 32 + 4 + 1)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitTicket<'info> {
    #[account(mut, has_one = project)]
    pub project: Account<'info, Project>,
    #[account(init, payer = reporter, space = 8 + 32 + 8 + 32 + 1 + 1)]
    pub ticket: Account<'info, Ticket>,
    #[account(mut)]
    pub reporter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AssignTickets<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(mut, has_one = project)]
    pub ticket_a: Account<'info, Ticket>,
    #[account(mut, has_one = project)]
    pub ticket_b: Account<'info, Ticket>,
}

#[account]
pub struct Project {
    pub manager: Pubkey,
    pub project_id: u32,
    pub name: String,
    pub ticket_count: u32,
    pub is_active: bool,
}

#[account]
pub struct Ticket {
    pub project: Pubkey,
    pub ticket_id: u64,
    pub reporter: Pubkey,
    pub priority: u8,
    pub status: TicketStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TicketStatus {
    Open,
    Assigned,
    InProgress,
    Closed,
}

#[error_code]
pub enum TicketError {
    #[msg("Ticket is already closed.")]
    TicketClosed,
}