use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf556mvTWf");

#[program]
pub mod transform_config_556 {
    use super::*;

    pub fn transform_config(ctx: Context<Ctx556>, amount: u64) -> Result<()> {
        let prev = ctx.accounts.record.field;
        let new_field = prev.checked_add(amount).ok_or(ErrorCode::Overflow)?;
        ctx.accounts.record.field = new_field;
        msg!("Case 556: field {} -> {}", prev, new_field);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx556<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record556>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record556 {
    pub owner: Pubkey,
    pub field: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow prevented")]
    Overflow,
}
