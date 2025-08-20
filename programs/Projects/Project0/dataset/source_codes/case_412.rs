use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf412mvTWf");

#[program]
pub mod adjust_credit_412 {
    use super::*;

    pub fn adjust_credit(ctx: Context<AdjustCreditCtx412>) -> Result<()> {
        require_keys_neq!(
            ctx.accounts.acc1.key(),
            ctx.accounts.acc2.key(),
            ErrorCode::DuplicateAccount
        );
        let tmp = ctx.accounts.acc1.amount;
        ctx.accounts.acc1.amount = ctx.accounts.acc2.amount;
        ctx.accounts.acc2.amount = tmp;
        msg!("Case 412: amounts swapped");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AdjustCreditCtx412<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, AdjustCreditRecord412>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, AdjustCreditRecord412>,
    pub owner: Signer<'info>,
}

#[account]
pub struct AdjustCreditRecord412 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
