use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf196mvTWf");

#[program]
pub mod tune_account_196 {
    use super::*;

    pub fn tune_account(ctx: Context<Ctx196>, delta: u64) -> Result<()> {
        let previous = ctx.accounts.target.value;
        let new_val = previous.checked_add(delta).unwrap();
        ctx.accounts.target.value = new_val;
        msg!("Case 196: value {} -> {}", previous, new_val);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx196<'info> {
    #[account(mut, has_one = owner)]
    pub target: Account<'info, Target196>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Target196 {
    pub owner: Pubkey,
    pub value: u64,
}
