
// 10. Lottery System with AccountInfo Pattern
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

declare_id!("LotterySystem111111111111111111111111111111111111");

#[program]
pub mod lottery_system {
    use super::*;
    
    pub fn initialize_lottery(ctx: Context<InitializeLottery>, ticket_price: u64, max_tickets: u64) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        lottery.authority = ctx.accounts.authority.key();
        lottery.ticket_price = ticket_price;
        lottery.max_tickets = max_tickets;
        lottery.tickets_sold = 0;
        lottery.is_active = true;
        lottery.winner = Pubkey::default();
        Ok(())
    }
    
    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        
        require!(lottery.is_active, LotteryError::LotteryInactive);
        require!(lottery.tickets_sold < lottery.max_tickets, LotteryError::SoldOut);
        
        // Validate token account belongs to participant
        let participant_key = ctx.accounts.participant.key();
        let token_account = &ctx.accounts.participant_token_account;
        
        require!(token_account.owner == participant_key, LotteryError::InvalidTokenAccount);
        require!(token_account.amount >= lottery.ticket_price, LotteryError::InsufficientFunds);
        
        // Transfer ticket price to lottery pool
        let cpi_accounts = anchor_spl::token::Transfer {
            from: ctx.accounts.participant_token_account.to_account_info(),
            to: ctx.accounts.lottery_pool.to_account_info(),
            authority: ctx.accounts.participant.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        anchor_spl::token::transfer(cpi_ctx, lottery.ticket_price)?;
        
        lottery.tickets_sold += 1;
        
        Ok(())
    }
    
    pub fn draw_winner<'info>(ctx: Context<'_, '_, '_, 'info, DrawWinner<'info>>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        
        require!(lottery.is_active, LotteryError::LotteryInactive);
        require!(lottery.tickets_sold > 0, LotteryError::NoTicketsSold);
        
        // Use clock and remaining accounts to generate randomness
        let clock = Clock::get()?;
        let mut seed = clock.unix_timestamp as u64;
        
        // Add entropy from remaining accounts (ticket holders)
        for account_info in ctx.remaining_accounts.iter() {
            // Validate account is from this program
            if account_info.owner == &crate::ID {
                seed = seed.wrapping_add(account_info.key().to_bytes()[0] as u64);
            }
        }
        
        let winner_index = seed % lottery.tickets_sold;
        
        // In a real implementation, you would map this to actual ticket holders
        // This is simplified for demonstration
        if let Some(winner_account) = ctx.remaining_accounts.get(winner_index as usize) {
            lottery.winner = winner_account.key();
        }
        
        lottery.is_active = false;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeLottery<'info> {
    #[account(init, payer = authority, space = 8 + 200)]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    pub participant_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lottery_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub participant: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DrawWinner<'info> {
    #[account(mut, constraint = lottery.authority == authority.key())]
    pub lottery: Account<'info, Lottery>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Lottery {
    pub authority: Pubkey,
    pub ticket_price: u64,
    pub max_tickets: u64,
    pub tickets_sold: u64,
    pub is_active: bool,
    pub winner: Pubkey,
}

#[error_code]
pub enum LotteryError {
    #[msg("Lottery is not active")]
    LotteryInactive,
    #[msg("All tickets sold")]
    SoldOut,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("No tickets sold")]
    NoTicketsSold,
}