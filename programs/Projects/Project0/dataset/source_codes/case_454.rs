use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf454mvTWf");

#[program]
pub mod increase_token_454 {
    use super::*;

    pub fn increase_token(ctx: Context<IncreaseTokenCtx454>) -> Result<()> {
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
        msg!("Case 454: max value is {}", maxv);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IncreaseTokenCtx454<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, IncreaseTokenRecord454>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, IncreaseTokenRecord454>,
    pub owner: Signer<'info>,
}

#[account]
pub struct IncreaseTokenRecord454 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
