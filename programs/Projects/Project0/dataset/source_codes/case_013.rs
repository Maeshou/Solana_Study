use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf013mvTWf");

#[program]
pub mod register_threshold_013 {
    use super::*;

    pub fn register_threshold(ctx: Context<Ctx013>, amount: u64) -> Result<()> {
        let data = ctx.accounts.storage.data;
        let rotated = data.rotate_right((amount % 64) as u32);
        ctx.accounts.storage.data = rotated;
        let summary = rotated.checked_add(amount).unwrap();
        msg!("Case 013: rotated {} + {} = {}", data, amount, summary);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx013<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage013>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage013 {
    pub authority: Pubkey,
    pub data: u64,
}
