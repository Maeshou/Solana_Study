use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf191mvTWf");

#[program]
pub mod settle_data_191 {
    use super::*;

    pub fn settle_data(ctx: Context<Ctx191>, delta: u64) -> Result<()> {
        let previous = ctx.accounts.target.value;
        let new_val = previous.checked_add(delta).unwrap();
        ctx.accounts.target.value = new_val;
        msg!("Case 191: value {} -> {}", previous, new_val);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx191<'info> {
    #[account(mut, has_one = owner)]
    pub target: Account<'info, Target191>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Target191 {
    pub owner: Pubkey,
    pub value: u64,
}
