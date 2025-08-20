use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf088mvTWf");

#[program]
pub mod allocate_parameter_088 {
    use super::*;

    pub fn allocate_parameter(ctx: Context<Ctx088>, amount: u64) -> Result<()> {
        let data = ctx.accounts.storage.data;
        let rotated = data.rotate_right((amount % 64) as u32);
        ctx.accounts.storage.data = rotated;
        let summary = rotated.checked_add(amount).unwrap();
        msg!("Case 088: rotated {} + {} = {}", data, amount, summary);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx088<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage088>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage088 {
    pub authority: Pubkey,
    pub data: u64,
}
