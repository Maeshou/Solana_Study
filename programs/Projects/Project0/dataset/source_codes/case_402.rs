use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf402mvTWf");

#[program]
pub mod adjust_credit_402 {
    use super::*;

    pub fn adjust_credit(ctx: Context<AdjustCreditCtx402>) -> Result<()> {
        require_keys_neq!(
            ctx.accounts.acc1.key(),
            ctx.accounts.acc2.key(),
            ErrorCode::DuplicateAccount
        );
        let tmp = ctx.accounts.acc1.amount;
        ctx.accounts.acc1.amount = ctx.accounts.acc2.amount;
        ctx.accounts.acc2.amount = tmp;
        msg!("Case 402: amounts swapped");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AdjustCreditCtx402<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, AdjustCreditRecord402>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, AdjustCreditRecord402>,
    pub owner: Signer<'info>,
}

#[account]
pub struct AdjustCreditRecord402 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
