use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf033mvTWf");

#[program]
pub mod register_threshold_033 {
    use super::*;

    pub fn register_threshold(ctx: Context<Ctx033>, amount: u64) -> Result<()> {
        let data = ctx.accounts.storage.data;
        let rotated = data.rotate_right((amount % 64) as u32);
        ctx.accounts.storage.data = rotated;
        let summary = rotated.checked_add(amount).unwrap();
        msg!("Case 033: rotated {} + {} = {}", data, amount, summary);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx033<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage033>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage033 {
    pub authority: Pubkey,
    pub data: u64,
}
