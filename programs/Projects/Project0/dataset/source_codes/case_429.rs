use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf429mvTWf");

#[program]
pub mod extend_dollar_429 {
    use super::*;

    pub fn extend_dollar(ctx: Context<ExtendDollarCtx429>) -> Result<()> {
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
        msg!("Case 429: max value is {}", maxv);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExtendDollarCtx429<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, ExtendDollarRecord429>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, ExtendDollarRecord429>,
    pub owner: Signer<'info>,
}

#[account]
pub struct ExtendDollarRecord429 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
