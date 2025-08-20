use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf068mvTWf");

#[program]
pub mod allocate_parameter_068 {
    use super::*;

    pub fn allocate_parameter(ctx: Context<Ctx068>, amount: u64) -> Result<()> {
        let data = ctx.accounts.storage.data;
        let rotated = data.rotate_right((amount % 64) as u32);
        ctx.accounts.storage.data = rotated;
        let summary = rotated.checked_add(amount).unwrap();
        msg!("Case 068: rotated {} + {} = {}", data, amount, summary);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx068<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage068>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage068 {
    pub authority: Pubkey,
    pub data: u64,
}
