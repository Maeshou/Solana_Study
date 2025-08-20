use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf038mvTWf");

#[program]
pub mod set_metadata_038 {
    use super::*;

    pub fn set_metadata(ctx: Context<Ctx038>, amount: u64) -> Result<()> {
        let data = ctx.accounts.storage.data;
        let rotated = data.rotate_right((amount % 64) as u32);
        ctx.accounts.storage.data = rotated;
        let summary = rotated.checked_add(amount).unwrap();
        msg!("Case 038: rotated {} + {} = {}", data, amount, summary);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx038<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage038>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage038 {
    pub authority: Pubkey,
    pub data: u64,
}
