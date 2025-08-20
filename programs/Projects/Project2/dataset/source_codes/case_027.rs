// =====================================
// 7. Escrow Program
// =====================================
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("77777777777777777777777777777777");

#[program]
pub mod secure_escrow {
    use super::*;

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        amount: u64,
        expected_amount: u64,
    ) -> Result<()> {
        // 厳密なowner checkを実装
        require!(
            ctx.accounts.initializer_deposit_token_account.owner == &token::ID,
            ErrorCode::InvalidInitializerTokenOwner
        );
        require!(
            ctx.accounts.escrow_account.owner == &token::ID,
            ErrorCode::InvalidEscrowTokenOwner
        );
        
        let escrow_info = ctx.accounts.escrow_state.to_account_info();
        require!(
            escrow_info.owner == ctx.program_id,
            ErrorCode::InvalidEscrowStateOwner
        );

        let escrow_state = &mut ctx.accounts.escrow_state;
        escrow_state.initializer = ctx.accounts.initializer.key();
        escrow_state.initializer_deposit_token_account = 
            ctx.accounts.initializer_deposit_token_account.key();
        escrow_state.initializer_receive_token_account = 
            ctx.accounts.initializer_receive_token_account.key();
        escrow_state.initializer_amount = amount;
        escrow_state.taker_amount = expected_amount;

        let transfer_instruction = Transfer {
            from: ctx.accounts.initializer_deposit_token_account.to_account_info(),
            to: ctx.accounts.escrow_account.to_account_info(),
            authority: ctx.accounts.initializer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );

        token::transfer(cpi_ctx, amount)
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        // 複数のowner checkでセキュリティを確保
        let escrow_state_info = ctx.accounts.escrow_state.to_account_info();
        require!(
            escrow_state_info.owner == ctx.program_id,
            ErrorCode::InvalidEscrowStateOwner
        );
        
        require!(
            ctx.accounts.taker_deposit_token_account.owner == &token::ID,
            ErrorCode::InvalidTakerTokenOwner
        );
        require!(
            ctx.accounts.taker_receive_token_account.owner == &token::ID,
            ErrorCode::InvalidTakerReceiveTokenOwner
        );

        let escrow_state = &ctx.accounts.escrow_state;

        // Taker → Initializer への転送
        let transfer_to_initializer = Transfer {
            from: ctx.accounts.taker_deposit_token_account.to_account_info(),
            to: ctx.accounts.initializer_receive_token_account.to_account_info(),
            authority: ctx.accounts.taker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_to_initializer,
        );

        token::transfer(cpi_ctx, escrow_state.taker_amount)?;

        // Escrow → Taker への転送
        let seeds = &[
            b"escrow",
            escrow_state.initializer.as_ref(),
            &[ctx.accounts.escrow_state.bump],
        ];
        let signer = &[&seeds[..]];

        let transfer_to_taker = Transfer {
            from: ctx.accounts.escrow_account.to_account_info(),
            to: ctx.accounts.taker_receive_token_account.to_account_info(),
            authority: ctx.accounts.escrow_state.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_to_taker,
            signer,
        );

        token::transfer(cpi_ctx, escrow_state.initializer_amount)
    }
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(
        init,
        payer = initializer,
        space = 8 + 32 + 32 + 32 + 8 + 8 + 1,
        seeds = [b"escrow", initializer.key().as_ref()],
        bump,
        constraint = escrow_state.to_account_info().owner == program_id
    )]
    pub escrow_state: Account<'info, EscrowState>,
    #[account(
        mut,
        constraint = initializer_deposit_token_account.owner == &token::ID
    )]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    #[account(constraint = initializer_receive_token_account.owner == &token::ID)]
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = escrow_account.owner == &token::ID
    )]
    pub escrow_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(
        mut,
        constraint = escrow_state.to_account_info().owner == program_id
    )]
    pub escrow_state: Account<'info, EscrowState>,
    #[account(
        mut,
        constraint = taker_deposit_token_account.owner == &token::ID
    )]
    pub taker_deposit_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = taker_receive_token_account.owner == &token::ID
    )]
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = initializer_receive_token_account.owner == &token::ID
    )]
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = escrow_account.owner == &token::ID
    )]
    pub escrow_account: Account<'info, TokenAccount>,
    pub taker: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct EscrowState {
    pub initializer: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
    pub bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid initializer token account owner")]
    InvalidInitializerTokenOwner,
    #[msg("Invalid escrow token account owner")]
    InvalidEscrowTokenOwner,
    #[msg("Invalid escrow state account owner")]
    InvalidEscrowStateOwner,
    #[msg("Invalid taker token account owner")]
    InvalidTakerTokenOwner,
    #[msg("Invalid taker receive token account owner")]
    InvalidTakerReceiveTokenOwner,
}