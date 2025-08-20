use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf106mvTWf");

#[program]
pub mod tune_account_106 {
    use super::*;

    pub fn tune_account(ctx: Context<Ctx106>, delta: u64) -> Result<()> {
        let previous = ctx.accounts.target.value;
        let new_val = previous.checked_add(delta).unwrap();
        ctx.accounts.target.value = new_val;
        msg!("Case 106: value {} -> {}", previous, new_val);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx106<'info> {
    #[account(mut, has_one = owner)]
    pub target: Account<'info, Target106>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Target106 {
    pub owner: Pubkey,
    pub value: u64,
}
