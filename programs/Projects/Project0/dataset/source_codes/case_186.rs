use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf186mvTWf");

#[program]
pub mod tune_account_186 {
    use super::*;

    pub fn tune_account(ctx: Context<Ctx186>, delta: u64) -> Result<()> {
        let previous = ctx.accounts.target.value;
        let new_val = previous.checked_add(delta).unwrap();
        ctx.accounts.target.value = new_val;
        msg!("Case 186: value {} -> {}", previous, new_val);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx186<'info> {
    #[account(mut, has_one = owner)]
    pub target: Account<'info, Target186>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Target186 {
    pub owner: Pubkey,
    pub value: u64,
}
