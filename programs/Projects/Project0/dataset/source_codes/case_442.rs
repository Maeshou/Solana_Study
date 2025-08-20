use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf442mvTWf");

#[program]
pub mod adjust_credit_442 {
    use super::*;

    pub fn adjust_credit(ctx: Context<AdjustCreditCtx442>) -> Result<()> {
        require_keys_neq!(
            ctx.accounts.acc1.key(),
            ctx.accounts.acc2.key(),
            ErrorCode::DuplicateAccount
        );
        let tmp = ctx.accounts.acc1.amount;
        ctx.accounts.acc1.amount = ctx.accounts.acc2.amount;
        ctx.accounts.acc2.amount = tmp;
        msg!("Case 442: amounts swapped");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AdjustCreditCtx442<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, AdjustCreditRecord442>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, AdjustCreditRecord442>,
    pub owner: Signer<'info>,
}

#[account]
pub struct AdjustCreditRecord442 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
