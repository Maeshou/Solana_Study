// =====================================
// 8. Lottery Program (AccountInfo使用)
// =====================================
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("88888888888888888888888888888888");

#[program]
pub mod secure_lottery {
    use super::*;

    pub fn create_lottery(
        ctx: Context<CreateLottery>,
        ticket_price: u64,
        end_time: i64,
    ) -> Result<()> {
        // AccountInfoを使った安全なowner check
        let lottery_account_info = ctx.accounts.lottery.to_account_info();
        require!(
            lottery_account_info.owner == ctx.program_id,
            ErrorCode::InvalidLotteryOwner
        );

        let lottery = &mut ctx.accounts.lottery;
        lottery.admin = ctx.accounts.admin.key();
        lottery.ticket_price = ticket_price;
        lottery.end_time = end_time;
        lottery.total_tickets = 0;
        lottery.is_active = true;
        lottery.winner = None;

        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        // 複数のowner checkを実装
        let lottery_info = ctx.accounts.lottery.to_account_info();
        require!(
            lottery_info.owner == ctx.program_id,
            ErrorCode::InvalidLotteryOwner
        );

        let ticket_info = ctx.accounts.ticket.to_account_info();
        require!(
            ticket_info.owner == ctx.program_id,
            ErrorCode::InvalidTicketOwner
        );

        require!(
            ctx.accounts.buyer_token_account.owner == &token::ID,
            ErrorCode::InvalidBuyerTokenOwner
        );

        let lottery = &mut ctx.accounts.lottery;
        require!(lottery.is_active, ErrorCode::LotteryNotActive);
        require!(
            Clock::get()?.unix_timestamp < lottery.end_time,
            ErrorCode::LotteryEnded
        );

        let ticket = &mut ctx.accounts.ticket;
        ticket.lottery = ctx.accounts.lottery.key();
        ticket.buyer = ctx.accounts.buyer.key();
        ticket.ticket_number = lottery.total_tickets;

        lottery.total_tickets += 1;

        let transfer_instruction = Transfer {
            from: ctx.accounts.buyer_token_account.to_account_info(),
            to: ctx.accounts.lottery_vault.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );

        token::transfer(cpi_ctx, lottery.ticket_price)
    }

    pub fn draw_winner(ctx: Context<DrawWinner>) -> Result<()> {
        // AccountInfoでのowner check実装
        let lottery_account_info = ctx.accounts.lottery.to_account_info();
        require!(
            lottery_account_info.owner == ctx.program_id,
            ErrorCode::InvalidLotteryOwner
        );

        let lottery = &mut ctx.accounts.lottery;
        require!(
            Clock::get()?.unix_timestamp >= lottery.end_time,
            ErrorCode::LotteryNotEnded
        );
        require!(lottery.is_active, ErrorCode::LotteryNotActive);

        // 簡単な乱数生成（実際の本番環境では安全な乱数生成が必要）
        let clock = Clock::get()?;
        let winning_number = (clock.unix_timestamp as u64) % lottery.total_tickets;
        
        lottery.winner = Some(winning_number);
        lottery.is_active = false;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateLottery<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 8 + 8 + 8 + 1 + 1 + 8,
        constraint = lottery.to_account_info().owner == program_id
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
        constraint = lottery.to_account_info().owner == program_id
    )]
    pub lottery: Account<'info, Lottery>,
    #[account(
        init,
        payer = buyer,
        space = 8 + 32 + 32 + 8,
        constraint = ticket.to_account_info().owner == program_id
    )]
    pub ticket: Account<'info, Ticket>,
    #[account(
        mut,
        constraint = buyer_token_account.owner == &token::ID
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = lottery_vault.owner == &token::ID
    )]
    pub lottery_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DrawWinner<'info> {
    #[account(
        mut,
        has_one = admin,
        constraint = lottery.to_account_info().owner == program_id
    )]
    pub lottery: Account<'info, Lottery>,
    pub admin: Signer<'info>,
}

#[account]
pub struct Lottery {
    pub admin: Pubkey,
    pub ticket_price: u64,
    pub end_time: i64,
    pub total_tickets: u64,
    pub is_active: bool,
    pub winner: Option<u64>,
}

#[account]
pub struct Ticket {
    pub lottery: Pubkey,
    pub buyer: Pubkey,
    pub ticket_number: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid lottery account owner")]
    InvalidLotteryOwner,
    #[msg("Invalid ticket account owner")]
    InvalidTicketOwner,
    #[msg("Invalid buyer token account owner")]
    InvalidBuyerTokenOwner,
    #[msg("Lottery is not active")]
    LotteryNotActive,
    #[msg("Lottery has ended")]
    LotteryEnded,
    #[msg("Lottery has not ended yet")]
    LotteryNotEnded,
}