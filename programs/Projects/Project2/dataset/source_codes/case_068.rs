// 8. Escrow Service with Milestone Payments
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("EscrowService111111111111111111111111111111111111");

#[program]
pub mod escrow_service {
    use super::*;
    
    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64, milestones: Vec<u64>) -> Result<()> {
        require!(milestones.len() > 0, EscrowError::NoMilestones);
        require!(milestones.iter().sum::<u64>() == amount, EscrowError::MilestoneAmountMismatch);
        
        let escrow = &mut ctx.accounts.escrow;
        escrow.client = ctx.accounts.client.key();
        escrow.provider = ctx.accounts.provider.key();
        escrow.total_amount = amount;
        escrow.milestones = milestones;
        escrow.completed_milestones = 0;
        escrow.is_active = true;
        
        // Transfer tokens to escrow account
        let cpi_accounts = anchor_spl::token::Transfer {
            from: ctx.accounts.client_token_account.to_account_info(),
            to: ctx.accounts.escrow_token_account.to_account_info(),
            authority: ctx.accounts.client.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        anchor_spl::token::transfer(cpi_ctx, amount)?;
        
        Ok(())
    }
    
    pub fn approve_milestone(ctx: Context<ApproveMilestone>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        
        require!(escrow.is_active, EscrowError::EscrowInactive);
        require!(escrow.completed_milestones < escrow.milestones.len() as u64, EscrowError::AllMilestonesCompleted);
        
        let milestone_amount = escrow.milestones[escrow.completed_milestones as usize];
        escrow.completed_milestones += 1;
        
        // Transfer milestone payment to provider
        let seeds = &[
            b"escrow",
            escrow.client.as_ref(),
            escrow.provider.as_ref(),
            &[ctx.bumps.escrow_token_account],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = anchor_spl::token::Transfer {
            from: ctx.accounts.escrow_token_account.to_account_info(),
            to: ctx.accounts.provider_token_account.to_account_info(),
            authority: ctx.accounts.escrow_token_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        anchor_spl::token::transfer(cpi_ctx, milestone_amount)?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(init, payer = client, space = 8 + 500, seeds = [b"escrow", client.key().as_ref(), provider.key().as_ref()], bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(init, payer = client, token::mint = token_mint, token::authority = escrow_token_account, seeds = [b"escrow", client.key().as_ref(), provider.key().as_ref()], bump)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = client_token_account.mint == token_mint.key())]
    pub client_token_account: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    /// CHECK: Provider pubkey for escrow creation
    pub provider: UncheckedAccount<'info>,
    #[account(mut)]
    pub client: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveMilestone<'info> {
    #[account(mut, seeds = [b"escrow", escrow.client.as_ref(), escrow.provider.as_ref()], bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut, seeds = [b"escrow", escrow.client.as_ref(), escrow.provider.as_ref()], bump)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub provider_token_account: Account<'info, TokenAccount>,
    pub client: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Escrow {
    pub client: Pubkey,
    pub provider: Pubkey,
    pub total_amount: u64,
    pub milestones: Vec<u64>,
    pub completed_milestones: u64,
    pub is_active: bool,
}

#[error_code]
pub enum EscrowError {
    #[msg("No milestones provided")]
    NoMilestones,
    #[msg("Milestone amounts don't match total")]
    MilestoneAmountMismatch,
    #[msg("Escrow is not active")]
    EscrowInactive,
    #[msg("All milestones completed")]
    AllMilestonesCompleted,
}
