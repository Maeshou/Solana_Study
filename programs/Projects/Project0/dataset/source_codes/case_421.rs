use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf421mvTWf");

#[program]
pub mod transfer_resource_421 {
    use super::*;

    pub fn transfer_resource(ctx: Context<TransferResourceCtx421>, delta: u64) -> Result<()> {
        // Ensure distinct accounts
        require_keys_neq!(
            ctx.accounts.acc1.key(),
            ctx.accounts.acc2.key(),
            ErrorCode::DuplicateAccount
        );
        // Update amounts
        ctx.accounts.acc1.amount = ctx.accounts.acc1.amount.checked_add(delta).unwrap();
        ctx.accounts.acc2.amount = ctx.accounts.acc2.amount.checked_add(delta).unwrap();
        msg!("Case 421: both accounts increased by {}", delta);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferResourceCtx421<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, TransferResourceRecord421>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, TransferResourceRecord421>,
    pub owner: Signer<'info>,
}

#[account]
pub struct TransferResourceRecord421 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
