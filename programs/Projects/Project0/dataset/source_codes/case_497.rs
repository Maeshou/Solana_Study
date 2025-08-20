use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf497mvTWf");

#[program]
pub mod modify_unit_497 {
    use super::*;

    pub fn modify_unit(ctx: Context<ModifyUnitCtx497>) -> Result<()> {
        require_keys_neq!(
            ctx.accounts.acc1.key(),
            ctx.accounts.acc2.key(),
            ErrorCode::DuplicateAccount
        );
        let tmp = ctx.accounts.acc1.amount;
        ctx.accounts.acc1.amount = ctx.accounts.acc2.amount;
        ctx.accounts.acc2.amount = tmp;
        msg!("Case 497: amounts swapped");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyUnitCtx497<'info> {
    #[account(mut, has_one = owner)]
    pub acc1: Account<'info, ModifyUnitRecord497>,
    #[account(mut, has_one = owner)]
    pub acc2: Account<'info, ModifyUnitRecord497>,
    pub owner: Signer<'info>,
}

#[account]
pub struct ModifyUnitRecord497 {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts not allowed")]
    DuplicateAccount,
}
