use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf133mvTWf");

#[program]
pub mod reset_data_133 {
    use super::*;

    pub fn reset_data(ctx: Context<Ctx133>) -> Result<()> {
        let num = ctx.accounts.storage.num;
        let rotated = num.rotate_left((ctx.accounts.user.key().to_bytes()[0] % 32) as u32);
        ctx.accounts.storage.num = rotated;
        msg!("Case 133: rotated {} -> {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx133<'info> {
    #[account(mut, has_one = owner)]
    pub storage: Account<'info, Storage133>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Storage133 {
    pub owner: Pubkey,
    pub num: u64,
}
