use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf126mvTWf");

#[program]
pub mod tune_account_126 {
    use super::*;

    pub fn tune_account(ctx: Context<Ctx126>, delta: u64) -> Result<()> {
        let previous = ctx.accounts.target.value;
        let new_val = previous.checked_add(delta).unwrap();
        ctx.accounts.target.value = new_val;
        msg!("Case 126: value {} -> {}", previous, new_val);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx126<'info> {
    #[account(mut, has_one = owner)]
    pub target: Account<'info, Target126>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Target126 {
    pub owner: Pubkey,
    pub value: u64,
}
