use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf123mvTWf");

#[program]
pub mod reset_data_123 {
    use super::*;

    pub fn reset_data(ctx: Context<Ctx123>) -> Result<()> {
        let num = ctx.accounts.storage.num;
        let rotated = num.rotate_left((ctx.accounts.user.key().to_bytes()[0] % 32) as u32);
        ctx.accounts.storage.num = rotated;
        msg!("Case 123: rotated {} -> {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx123<'info> {
    #[account(mut, has_one = owner)]
    pub storage: Account<'info, Storage123>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Storage123 {
    pub owner: Pubkey,
    pub num: u64,
}
