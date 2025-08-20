// =============================================================================
// 9. Lottery System with Admin Controls and Owner Verification
// =============================================================================
#[program]
pub mod secure_lottery {
    use super::*;

    pub fn create_lottery(ctx: Context<CreateLottery>, ticket_price: u64, max_tickets: u64) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        lottery.admin = ctx.accounts.admin.key();
        lottery.ticket_price = ticket_price;
        lottery.max_tickets = max_tickets;
        lottery.tickets_sold = 0;
        lottery.is_active = true;
        lottery.winner = None;
        lottery.bump = *ctx.bumps.get("lottery").unwrap();
        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        let ticket = &mut ctx.accounts.ticket;
        
        require!(lottery.is_active, LotteryError::LotteryNotActive);
        require!(lottery.tickets_sold < lottery.max_tickets, LotteryError::SoldOut);
        
        ticket.lottery = lottery.key();
        ticket.owner = ctx.accounts.buyer.key();
        ticket.ticket_number = lottery.tickets_sold;
        ticket.bump = *ctx.bumps.get("ticket").unwrap();
        
        lottery.tickets_sold += 1;
        
        // Payment handling would go here
        Ok(())
    }

    pub fn draw_winner(ctx: Context<DrawWinner>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        
        require!(lottery.is_active, LotteryError::LotteryNotActive);
        require!(lottery.tickets_sold > 0, LotteryError::NoTicketsSold);
        
        // Simple random winner selection (in production, use proper randomness)
        let winner_ticket = lottery.tickets_sold % 10;
        lottery.winner = Some(winner_ticket);
        lottery.is_active = false;
        
        Ok(())
    }
}

#[account]
pub struct Lottery {
    pub admin: Pubkey,
    pub ticket_price: u64,
    pub max_tickets: u64,
    pub tickets_sold: u64,
    pub is_active: bool,
    pub winner: Option<u64>,
    pub bump: u8,
}

#[account]
pub struct Ticket {
    pub lottery: Pubkey,
    pub owner: Pubkey,
    pub ticket_number: u64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreateLottery<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 8 + 8 + 8 + 1 + 1 + 8 + 1,
        seeds = [b"lottery", admin.key().as_ref()],
        bump
    )]
    pub lottery: Account<'info, Lottery>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(
        mut,
        seeds = [b"lottery", lottery.admin.as_ref()],
        bump = lottery.bump
    )]
    pub lottery: Account<'info, Lottery>,
    
    #[account(
        init,
        payer = buyer,
        space = 8 + 32 + 32 + 8 + 1,
        seeds = [b"ticket", lottery.key().as_ref(), &lottery.tickets_sold.to_le_bytes()],
        bump
    )]
    pub ticket: Account<'info, Ticket>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DrawWinner<'info> {
    #[account(
        mut,
        seeds = [b"lottery", admin.key().as_ref()],
        bump = lottery.bump,
        constraint = lottery.admin == admin.key()
    )]
    pub lottery: Account<'info, Lottery>,
    
    pub admin: Signer<'info>,
}

#[error_code]
pub enum LotteryError {
    #[msg("Lottery is not active")]
    LotteryNotActive,
    #[msg("All tickets have been sold")]
    SoldOut,
    #[msg("No tickets have been sold")]
    NoTicketsSold,
}
