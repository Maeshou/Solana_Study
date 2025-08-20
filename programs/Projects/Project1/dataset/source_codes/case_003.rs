use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf003mvTWf");

#[program]
pub mod refresh_account_003 {
    use super::*;

    pub fn refresh_account(ctx: Context<Ctx003>, amount: u64) -> Result<()> {
        let data = ctx.accounts.storage.data;
        let rotated = data.rotate_right((amount % 64) as u32);
        ctx.accounts.storage.data = rotated;
        let summary = rotated.checked_add(amount).unwrap();
        msg!("Case 003: rotated {} + {} = {}", data, amount, summary);
        
        Ok(())
    }

    pub fn add_to_data(ctx: Context<Ctx003>, addend: u64) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);
        ctx.accounts.storage.data = ctx.accounts.storage.data.checked_add(addend).unwrap();
        msg!("Data incremented by {} to {}", addend, ctx.accounts.storage.data);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub data: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access")]
    Unauthorized,
}
