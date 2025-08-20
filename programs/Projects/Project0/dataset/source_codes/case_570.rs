use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf570mvTWf");

#[program]
pub mod adjust_token_570 {
    use super::*;

    pub fn adjust_token(ctx: Context<Ctx570>, factor: u64) -> Result<()> {
        let base = ctx.accounts.record.amount;
        let result = base.checked_mul(factor).ok_or(ErrorCode::Overflow)?;
        ctx.accounts.record.amount = result;
        msg!("Case 570: amount {} * {} = {}", base, factor, result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx570<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record570>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record570 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow prevented")]
    Overflow,
}
