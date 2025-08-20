use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf131mvTWf");

#[program]
pub mod settle_data_131 {
    use super::*;

    pub fn settle_data(ctx: Context<Ctx131>, delta: u64) -> Result<()> {
        let previous = ctx.accounts.target.value;
        let new_val = previous.checked_add(delta).unwrap();
        ctx.accounts.target.value = new_val;
        msg!("Case 131: value {} -> {}", previous, new_val);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx131<'info> {
    #[account(mut, has_one = owner)]
    pub target: Account<'info, Target131>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Target131 {
    pub owner: Pubkey,
    pub value: u64,
}
