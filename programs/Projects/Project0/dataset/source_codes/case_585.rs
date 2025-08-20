use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf585mvTWf");

#[program]
pub mod multiply_profile_585 {
    use super::*;

    pub fn multiply_profile(ctx: Context<Ctx585>, factor: u64) -> Result<()> {
        let base = ctx.accounts.record.amount;
        let result = base.checked_mul(factor).ok_or(ErrorCode::Overflow)?;
        ctx.accounts.record.amount = result;
        msg!("Case 585: amount {} * {} = {}", base, factor, result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx585<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record585>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record585 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow prevented")]
    Overflow,
}
