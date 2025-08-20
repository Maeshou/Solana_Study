// =============================================================================
// 7. Escrow Service with AccountInfo Safety Checks
// =============================================================================
#[program]
pub mod secure_escrow {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow_account;
        escrow.buyer = ctx.accounts.buyer.key();
        escrow.seller = *ctx.accounts.seller.key;
        escrow.amount = amount;
        escrow.is_completed = false;
        escrow.bump = *ctx.bumps.get("escrow_account").unwrap();
        
        // Transfer funds to escrow
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.buyer.key(),
            &escrow.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.escrow_account.to_account_info(),
            ],
        )?;
        
        Ok(())
    }

    pub fn complete_escrow(ctx: Context<CompleteEscrow>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow_account;
        
        // Transfer funds from escrow to seller
        **ctx.accounts.escrow_account.to_account_info().lamports.borrow_mut() -= escrow.amount;
        **ctx.accounts.seller.lamports.borrow_mut() += escrow.amount;
        
        escrow.is_completed = true;
        Ok(())
    }
}

#[account]
pub struct EscrowAccount {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub is_completed: bool,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(
        init,
        payer = buyer,
        space = 8 + 32 + 32 + 8 + 1 + 1,
        seeds = [b"escrow", buyer.key().as_ref(), seller.key.as_ref()],
        bump
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    /// CHECK: This account is properly validated through seeds and stored in escrow
    pub seller: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteEscrow<'info> {
    #[account(
        mut,
        seeds = [b"escrow", escrow_account.buyer.as_ref(), escrow_account.seller.as_ref()],
        bump = escrow_account.bump,
        constraint = escrow_account.buyer == buyer.key(),
        constraint = !escrow_account.is_completed @ EscrowError::AlreadyCompleted
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    
    pub buyer: Signer<'info>,
    
    /// CHECK: This account's pubkey is verified against the escrow's seller field
    #[account(
        mut,
        constraint = seller.key() == escrow_account.seller
    )]
    pub seller: AccountInfo<'info>,
}

#[error_code]
pub enum EscrowError {
    #[msg("Escrow already completed")]
    AlreadyCompleted,
}
