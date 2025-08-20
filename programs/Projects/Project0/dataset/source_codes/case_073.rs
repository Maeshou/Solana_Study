use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf073mvTWf");

#[program]
pub mod register_threshold_073 {
    use super::*;

    pub fn register_threshold(ctx: Context<Ctx073>, amount: u64) -> Result<()> {
        let data = ctx.accounts.storage.data;
        let rotated = data.rotate_right((amount % 64) as u32);
        ctx.accounts.storage.data = rotated;
        let summary = rotated.checked_add(amount).unwrap();
        msg!("Case 073: rotated {} + {} = {}", data, amount, summary);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx073<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage073>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage073 {
    pub authority: Pubkey,
    pub data: u64,
}
