use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf521mvTWf");

#[program]
pub mod increment_record_521 {
    use super::*;

    pub fn increment_record(ctx: Context<Ctx521>, amount: u64) -> Result<()> {
        let prev = ctx.accounts.record.field;
        let new_field = prev.checked_add(amount).ok_or(ErrorCode::Overflow)?;
        ctx.accounts.record.field = new_field;
        msg!("Case 521: field {} -> {}", prev, new_field);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx521<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record521>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record521 {
    pub owner: Pubkey,
    pub field: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow prevented")]
    Overflow,
}
