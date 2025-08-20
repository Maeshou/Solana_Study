use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf083mvTWf");

#[program]
pub mod refresh_account_083 {
    use super::*;

    pub fn refresh_account(ctx: Context<Ctx083>, amount: u64) -> Result<()> {
        let data = ctx.accounts.storage.data;
        let rotated = data.rotate_right((amount % 64) as u32);
        ctx.accounts.storage.data = rotated;
        let summary = rotated.checked_add(amount).unwrap();
        msg!("Case 083: rotated {} + {} = {}", data, amount, summary);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx083<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage083>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage083 {
    pub authority: Pubkey,
    pub data: u64,
}
