use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf459mvTWf");

#[program]
pub mod extend_dollar_459 {
    use super::*;

    pub fn extend_dollar(ctx: Context<ExtendDollarCtx459>) -> Result<()> {
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
        msg!("Case 459: max value is {}", maxv);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExtendDollarCtx459<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, ExtendDollarRecord459>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, ExtendDollarRecord459>,
    pub owner: Signer<'info>,
}

#[account]
pub struct ExtendDollarRecord459 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
