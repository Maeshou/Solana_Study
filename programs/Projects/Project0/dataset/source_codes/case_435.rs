use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf435mvTWf");

#[program]
pub mod boost_point_435 {
    use super::*;

    pub fn boost_point(ctx: Context<BoostPointCtx435>) -> Result<()> {
        require_keys_neq!(
            ctx.accounts.acc1.key(),
            ctx.accounts.acc2.key(),
            ErrorCode::DuplicateAccount
        );
        let diff = if ctx.accounts.acc1.amount > ctx.accounts.acc2.amount {
            ctx.accounts.acc1.amount - ctx.accounts.acc2.amount
        } else {
            ctx.accounts.acc2.amount - ctx.accounts.acc1.amount
        };
        ctx.accounts.acc1.amount = diff;
        ctx.accounts.acc2.amount = diff;
        msg!("Case 435: diff value {}", diff);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BoostPointCtx435<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, BoostPointRecord435>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, BoostPointRecord435>,
    pub owner: Signer<'info>,
}

#[account]
pub struct BoostPointRecord435 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
