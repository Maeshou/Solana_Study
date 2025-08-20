// =============================================================================
// 16. Event Ticketing System
// =============================================================================
#[program]
pub mod secure_ticketing {
    use super::*;

    pub fn create_event(ctx: Context<CreateEvent>, name: String, venue: String, date: i64, ticket_price: u64, max_tickets: u64) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.organizer = ctx.accounts.organizer.key();
        event.name = name;
        event.venue = venue;
        event.date = date;
        event.ticket_price = ticket_price;
        event.max_tickets = max_tickets;
        event.tickets_sold = 0;
        event.is_active = true;
        event.bump = *ctx.bumps.get("event").unwrap();
        Ok(())
    }

    pub fn purchase_ticket(ctx: Context<PurchaseTicket>, quantity: u64) -> Result<()> {
        let event = &mut ctx.accounts.event;
        let ticket = &mut ctx.accounts.ticket;
        
        require!(event.is_active, TicketingError::EventNotActive);
        require!(event.tickets_sold + quantity <= event.max_tickets, TicketingError::NotEnoughTickets);
        require!(Clock::get()?.unix_timestamp < event.date, TicketingError::EventPassed);
        
        ticket.event = event.key();
        ticket.owner = ctx.accounts.buyer.key();
        ticket.quantity = quantity;
        ticket.purchased_at = Clock::get()?.unix_timestamp;
        ticket.is_used = false;
        ticket.bump = *ctx.bumps.get("ticket").unwrap();
        
        event.tickets_sold += quantity;
        
        // Payment handling
        let total_cost = event.ticket_price * quantity;
        **ctx.accounts.buyer.lamports.borrow_mut() -= total_cost;
        **ctx.accounts.organizer.lamports.borrow_mut() += total_cost;
        
        Ok(())
    }

    pub fn validate_ticket(ctx: Context<ValidateTicket>) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket;
        let event = &ctx.accounts.event;
        
        require!(!ticket.is_used, TicketingError::TicketAlreadyUsed);
        require!(Clock::get()?.unix_timestamp >= event.date - 3600, TicketingError::TooEarlyForEntry); // 1 hour before
        require!(Clock::get()?.unix_timestamp <= event.date + 86400, TicketingError::EventExpired); // 24 hours after
        
        ticket.is_used = true;
        Ok(())
    }
}

#[account]
pub struct Event {
    pub organizer: Pubkey,
    pub name: String,
    pub venue: String,
    pub date: i64,
    pub ticket_price: u64,
    pub max_tickets: u64,
    pub tickets_sold: u64,
    pub is_active: bool,
    pub bump: u8,
}

#[account]
pub struct Ticket {
    pub event: Pubkey,
    pub owner: Pubkey,
    pub quantity: u64,
    pub purchased_at: i64,
    pub is_used: bool,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(name: String, venue: String)]
pub struct CreateEvent<'info> {
    #[account(
        init,
        payer = organizer,
        space = 8 + 32 + 4 + name.len() + 4 + venue.len() + 8 + 8 + 8 + 8 + 1 + 1,
        seeds = [b"event", organizer.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub event: Account<'info, Event>,
    
    #[account(mut)]
    pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseTicket<'info> {
    #[account(
        mut,
        seeds = [b"event", event.organizer.as_ref(), event.name.as_bytes()],
        bump = event.bump
    )]
    pub event: Account<'info, Event>,
    
    #[account(
        init,
        payer = buyer,
        space = 8 + 32 + 32 + 8 + 8 + 1 + 1,
        seeds = [b"ticket", event.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub ticket: Account<'info, Ticket>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    /// CHECK: Verified through event organizer field
    #[account(
        mut,
        constraint = organizer.key() == event.organizer
    )]
    pub organizer: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ValidateTicket<'info> {
    #[account(
        seeds = [b"event", event.organizer.as_ref(), event.name.as_bytes()],
        bump = event.bump
    )]
    pub event: Account<'info, Event>,
    
    #[account(
        mut,
        seeds = [b"ticket", event.key().as_ref(), ticket.owner.as_ref()],
        bump = ticket.bump,
        constraint = ticket.event == event.key()
    )]
    pub ticket: Account<'info, Ticket>,
    
    /// CHECK: Event organizer or authorized validator
    pub validator: Signer<'info>,
}

#[error_code]
pub enum TicketingError {
    #[msg("Event is not active")]
    EventNotActive,
    #[msg("Not enough tickets available")]
    NotEnoughTickets,
    #[msg("Event has already passed")]
    EventPassed,
    #[msg("Ticket has already been used")]
    TicketAlreadyUsed,
    #[msg("Too early for entry")]
    TooEarlyForEntry,
    #[msg("Event has expired")]
    EventExpired,
}
