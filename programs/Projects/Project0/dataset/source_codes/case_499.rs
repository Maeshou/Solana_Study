use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf499mvTWf");

#[program]
pub mod extend_dollar_499 {
    use super::*;

    pub fn extend_dollar(ctx: Context<ExtendDollarCtx499>) -> Result<()> {
        require_keys_neq!(
            ctx.accounts.acc1.key(),
            ctx.accounts.acc2.key(),
            ErrorCode::DuplicateAccount
        );
        let maxv = if ctx.accounts.acc1.amount > ctx.accounts.acc2.amount {
            ctx.accounts.acc1.amount
        } else {
            ctx.accounts.acc2.amount
        };
        ctx.accounts.acc1.amount = maxv;
        ctx.accounts.acc2.amount = maxv;
        msg!("Case 499: max value is {}", maxv);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExtendDollarCtx499<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, ExtendDollarRecord499>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, ExtendDollarRecord499>,
    pub owner: Signer<'info>,
}

#[account]
pub struct ExtendDollarRecord499 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
