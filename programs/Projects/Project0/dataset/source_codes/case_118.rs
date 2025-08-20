use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf118mvTWf");

#[program]
pub mod refresh_account_118 {
    use super::*;

    pub fn refresh_account(ctx: Context<Ctx118>) -> Result<()> {
        let num = ctx.accounts.storage.num;
        let rotated = num.rotate_left((ctx.accounts.user.key().to_bytes()[0] % 32) as u32);
        ctx.accounts.storage.num = rotated;
        msg!("Case 118: rotated {} -> {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx118<'info> {
    #[account(mut, has_one = owner)]
    pub storage: Account<'info, Storage118>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Storage118 {
    pub owner: Pubkey,
    pub num: u64,
}
