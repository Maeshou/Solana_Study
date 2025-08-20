use anchor_lang::prelude::*;
use anchor_lang::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpW5z6y7x8w9v0u1t2s3r4q5p6o7n");

#[program]
pub mod escrow_contract {
    use super::*;

    /// エスクローアカウントの初期化と資金デポジット
    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        amount: u64,
        deadline: i64,
        bump: u8,
    ) -> ProgramResult {
        let escrow = &mut ctx.accounts.escrow;
        escrow.buyer = *ctx.accounts.buyer.key;
        escrow.seller = *ctx.accounts.seller.key;
        escrow.amount = amount;
        escrow.deadline = deadline;
        escrow.bump = bump;

        // Buyer から Escrow PDA へ lamports を移動
        let buyer_lamports = ctx.accounts.buyer.to_account_info().lamports();
        require!(buyer_lamports >= amount, ErrorCode::InsufficientFunds);
        **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? =
            buyer_lamports.checked_sub(amount).ok_or(ErrorCode::Overflow)?;
        **ctx.accounts.escrow.to_account_info().try_borrow_mut_lamports()? =
            ctx.accounts.escrow.to_account_info().lamports()
                .checked_add(amount)
                .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }

    /// Seller に資金リリース（デッドライン前でも可）
    pub fn release_funds(
        ctx: Context<ReleaseFunds>,
    ) -> ProgramResult {
        let escrow = &ctx.accounts.escrow;
        require!(escrow.seller == *ctx.accounts.seller.key, ErrorCode::Unauthorized);

        // Escrow PDA から Seller へ lamports を移す
        let pot = ctx.accounts.escrow.to_account_info().lamports();
        **ctx.accounts.escrow.to_account_info().try_borrow_mut_lamports()? =
            pot.checked_sub(escrow.amount).ok_or(ErrorCode::InsufficientFunds)?;
        **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? =
            ctx.accounts.seller.to_account_info().lamports()
                .checked_add(escrow.amount)
                .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }

    /// Buyer に資金返却（デッドライン超過後）
    pub fn refund(
        ctx: Context<Refund>,
    ) -> ProgramResult {
        let escrow = &ctx.accounts.escrow;
        let now = Clock::get()?.unix_timestamp;
        require!(now >= escrow.deadline, ErrorCode::TooEarly);
        require!(escrow.buyer == *ctx.accounts.buyer.key, ErrorCode::Unauthorized);

        // Escrow PDA から Buyer へ lamports を戻す
        let pot = ctx.accounts.escrow.to_account_info().lamports();
        **ctx.accounts.escrow.to_account_info().try_borrow_mut_lamports()? =
            pot.checked_sub(escrow.amount).ok_or(ErrorCode::InsufficientFunds)?;
        **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? =
            ctx.accounts.buyer.to_account_info().lamports()
                .checked_add(escrow.amount)
                .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(amount: u64, deadline: i64, bump: u8)]
pub struct InitializeEscrow<'info> {
    #[account(
        init,
        seeds = [b"escrow", buyer.key().as_ref(), seller.key().as_ref()],
        bump = bump,
        payer = buyer,
        space = 8 + 32 + 32 + 8 + 8 + 1,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: リリース用
    pub seller: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ReleaseFunds<'info> {
    #[account(
        mut,
        seeds = [b"escrow", escrow.buyer.as_ref(), escrow.seller.as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub seller: Signer<'info>,
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(
        mut,
        seeds = [b"escrow", escrow.buyer.as_ref(), escrow.seller.as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub buyer: Signer<'info>,
}

#[account]
pub struct Escrow {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub deadline: i64,
    pub bump: u8,
}

#[error]
pub enum ErrorCode {
    #[msg("You are not authorized.")]
    Unauthorized,
    #[msg("Insufficient funds.")]
    InsufficientFunds,
    #[msg("Too early to refund.")]
    TooEarly,
    #[msg("Arithmetic overflow.")]
    Overflow,
}
