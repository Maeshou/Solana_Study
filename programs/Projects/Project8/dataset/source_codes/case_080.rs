// 8) stake_bank: Lamports デポジット／返金（PDA署名は不要設計、運営署名で制御）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_instruction, program::invoke};

declare_id!("St4k3B4nk888888888888888888888888888888");

#[program]
pub mod stake_bank {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let ix = system_instruction::transfer(
            &ctx.accounts.player.key(),
            &ctx.accounts.escrow.key(),
            amount,
        );
        invoke(
            &ix,
            &[
                ctx.accounts.player.to_account_info(),
                ctx.accounts.escrow.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        ctx.accounts.bank_state.total_locked =
            ctx.accounts.bank_state.total_locked.saturating_add(amount);
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>, amount: u64) -> Result<()> {
        require!(ctx.accounts.operator.is_signer, BankError::OperatorRequired);
        let bal = **ctx.accounts.escrow.to_account_info().lamports.borrow();
        require!(bal >= amount, BankError::InsufficientEscrow);
        **ctx.accounts.escrow.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.recipient.to_account_info().try_borrow_mut_lamports()? += amount;
        ctx.accounts.bank_state.total_locked =
            ctx.accounts.bank_state.total_locked.saturating_sub(amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        init,
        payer = player,
        space = 8 + BankState::LEN,
        seeds = [b"bank_state", player.key().as_ref()],
        bump
    )]
    pub bank_state: Account<'info, BankState>,
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub escrow: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub bank_state: Account<'info, BankState>,
    pub operator: Signer<'info>,
    #[account(mut)]
    pub escrow: SystemAccount<'info>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
}

#[account]
pub struct BankState { pub total_locked: u64 }
impl BankState { pub const LEN: usize = 8; }

#[error_code]
pub enum BankError {
    #[msg("Operator signature required.")] OperatorRequired,
    #[msg("Escrow balance is insufficient.")] InsufficientEscrow,
}
