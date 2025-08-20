use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf520mvTWf");

#[program]
pub mod adjust_token_520 {
    use super::*;

    pub fn adjust_token(ctx: Context<Ctx520>, factor: u64) -> Result<()> {
        let base = ctx.accounts.record.amount;
        let result = base.checked_mul(factor).ok_or(ErrorCode::Overflow)?;
        ctx.accounts.record.amount = result;
        msg!("Case 520: amount {} * {} = {}", base, factor, result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx520<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record520>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record520 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow prevented")]
    Overflow,
}
