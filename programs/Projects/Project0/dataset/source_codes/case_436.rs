use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf436mvTWf");

#[program]
pub mod amend_fund_436 {
    use super::*;

    pub fn amend_fund(ctx: Context<AmendFundCtx436>, delta: u64) -> Result<()> {
        // Ensure distinct accounts
        require_keys_neq!(
            ctx.accounts.acc1.key(),
            ctx.accounts.acc2.key(),
            ErrorCode::DuplicateAccount
        );
        // Update amounts
        ctx.accounts.acc1.amount = ctx.accounts.acc1.amount.checked_add(delta).unwrap();
        ctx.accounts.acc2.amount = ctx.accounts.acc2.amount.checked_add(delta).unwrap();
        msg!("Case 436: both accounts increased by {}", delta);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AmendFundCtx436<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, AmendFundRecord436>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, AmendFundRecord436>,
    pub owner: Signer<'info>,
}

#[account]
pub struct AmendFundRecord436 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
