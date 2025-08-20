use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf078mvTWf");

#[program]
pub mod set_metadata_078 {
    use super::*;

    pub fn set_metadata(ctx: Context<Ctx078>, amount: u64) -> Result<()> {
        let data = ctx.accounts.storage.data;
        let rotated = data.rotate_right((amount % 64) as u32);
        ctx.accounts.storage.data = rotated;
        let summary = rotated.checked_add(amount).unwrap();
        msg!("Case 078: rotated {} + {} = {}", data, amount, summary);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx078<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage078>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage078 {
    pub authority: Pubkey,
    pub data: u64,
}
