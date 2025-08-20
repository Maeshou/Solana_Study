use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf121mvTWf");

#[program]
pub mod settle_data_121 {
    use super::*;

    pub fn settle_data(ctx: Context<Ctx121>, delta: u64) -> Result<()> {
        let previous = ctx.accounts.target.value;
        let new_val = previous.checked_add(delta).unwrap();
        ctx.accounts.target.value = new_val;
        msg!("Case 121: value {} -> {}", previous, new_val);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx121<'info> {
    #[account(mut, has_one = owner)]
    pub target: Account<'info, Target121>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Target121 {
    pub owner: Pubkey,
    pub value: u64,
}
