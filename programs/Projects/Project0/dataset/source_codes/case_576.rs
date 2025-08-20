use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf576mvTWf");

#[program]
pub mod transform_config_576 {
    use super::*;

    pub fn transform_config(ctx: Context<Ctx576>, amount: u64) -> Result<()> {
        let prev = ctx.accounts.record.field;
        let new_field = prev.checked_add(amount).ok_or(ErrorCode::Overflow)?;
        ctx.accounts.record.field = new_field;
        msg!("Case 576: field {} -> {}", prev, new_field);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx576<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record576>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record576 {
    pub owner: Pubkey,
    pub field: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow prevented")]
    Overflow,
}
