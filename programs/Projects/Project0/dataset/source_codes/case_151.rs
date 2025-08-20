use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf151mvTWf");

#[program]
pub mod settle_data_151 {
    use super::*;

    pub fn settle_data(ctx: Context<Ctx151>, delta: u64) -> Result<()> {
        let previous = ctx.accounts.target.value;
        let new_val = previous.checked_add(delta).unwrap();
        ctx.accounts.target.value = new_val;
        msg!("Case 151: value {} -> {}", previous, new_val);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx151<'info> {
    #[account(mut, has_one = owner)]
    pub target: Account<'info, Target151>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Target151 {
    pub owner: Pubkey,
    pub value: u64,
}
