use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf546mvTWf");

#[program]
pub mod transform_config_546 {
    use super::*;

    pub fn transform_config(ctx: Context<Ctx546>, amount: u64) -> Result<()> {
        let prev = ctx.accounts.record.field;
        let new_field = prev.checked_add(amount).ok_or(ErrorCode::Overflow)?;
        ctx.accounts.record.field = new_field;
        msg!("Case 546: field {} -> {}", prev, new_field);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx546<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record546>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record546 {
    pub owner: Pubkey,
    pub field: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow prevented")]
    Overflow,
}
