use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf043mvTWf");

#[program]
pub mod refresh_account_043 {
    use super::*;

    pub fn refresh_account(ctx: Context<Ctx043>, amount: u64) -> Result<()> {
        let data = ctx.accounts.storage.data;
        let rotated = data.rotate_right((amount % 64) as u32);
        ctx.accounts.storage.data = rotated;
        let summary = rotated.checked_add(amount).unwrap();
        msg!("Case 043: rotated {} + {} = {}", data, amount, summary);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx043<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage043>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage043 {
    pub authority: Pubkey,
    pub data: u64,
}
