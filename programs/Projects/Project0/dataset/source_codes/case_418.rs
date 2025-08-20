use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf418mvTWf");

#[program]
pub mod sync_value_418 {
    use super::*;

    pub fn sync_value(ctx: Context<SyncValueCtx418>) -> Result<()> {
        require_keys_neq!(
            ctx.accounts.acc1.key(),
            ctx.accounts.acc2.key(),
            ErrorCode::DuplicateAccount
        );
        let sum = ctx.accounts.acc1.amount.checked_add(ctx.accounts.acc2.amount).unwrap();
        ctx.accounts.acc1.amount = sum;
        ctx.accounts.acc2.amount = 0;
        msg!("Case 418: sum is {}", sum);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SyncValueCtx418<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, SyncValueRecord418>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, SyncValueRecord418>,
    pub owner: Signer<'info>,
}

#[account]
pub struct SyncValueRecord418 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
