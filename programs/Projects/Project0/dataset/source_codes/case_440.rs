use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf440mvTWf");

#[program]
pub mod recharge_asset_440 {
    use super::*;

    pub fn recharge_asset(ctx: Context<RechargeAssetCtx440>) -> Result<()> {
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
        msg!("Case 440: diff value {}", diff);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RechargeAssetCtx440<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, RechargeAssetRecord440>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, RechargeAssetRecord440>,
    pub owner: Signer<'info>,
}

#[account]
pub struct RechargeAssetRecord440 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
