use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf462mvTWf");

#[program]
pub mod adjust_credit_462 {
    use super::*;

    pub fn adjust_credit(ctx: Context<AdjustCreditCtx462>) -> Result<()> {
        require_keys_neq!(
            ctx.accounts.acc1.key(),
            ctx.accounts.acc2.key(),
            ErrorCode::DuplicateAccount
        );
        let tmp = ctx.accounts.acc1.amount;
        ctx.accounts.acc1.amount = ctx.accounts.acc2.amount;
        ctx.accounts.acc2.amount = tmp;
        msg!("Case 462: amounts swapped");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AdjustCreditCtx462<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, AdjustCreditRecord462>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, AdjustCreditRecord462>,
    pub owner: Signer<'info>,
}

#[account]
pub struct AdjustCreditRecord462 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
