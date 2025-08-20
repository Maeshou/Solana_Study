use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf565mvTWf");

#[program]
pub mod multiply_profile_565 {
    use super::*;

    pub fn multiply_profile(ctx: Context<Ctx565>, factor: u64) -> Result<()> {
        let base = ctx.accounts.record.amount;
        let result = base.checked_mul(factor).ok_or(ErrorCode::Overflow)?;
        ctx.accounts.record.amount = result;
        msg!("Case 565: amount {} * {} = {}", base, factor, result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx565<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record565>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record565 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow prevented")]
    Overflow,
}
