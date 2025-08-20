use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf433mvTWf");

#[program]
pub mod update_share_433 {
    use super::*;

    pub fn update_share(ctx: Context<UpdateShareCtx433>) -> Result<()> {
        require_keys_neq!(
            ctx.accounts.acc1.key(),
            ctx.accounts.acc2.key(),
            ErrorCode::DuplicateAccount
        );
        let sum = ctx.accounts.acc1.amount.checked_add(ctx.accounts.acc2.amount).unwrap();
        ctx.accounts.acc1.amount = sum;
        ctx.accounts.acc2.amount = 0;
        msg!("Case 433: sum is {}", sum);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateShareCtx433<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, UpdateShareRecord433>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, UpdateShareRecord433>,
    pub owner: Signer<'info>,
}

#[account]
pub struct UpdateShareRecord433 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
