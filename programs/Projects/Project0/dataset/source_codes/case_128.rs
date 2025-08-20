use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf128mvTWf");

#[program]
pub mod refresh_account_128 {
    use super::*;

    pub fn refresh_account(ctx: Context<Ctx128>) -> Result<()> {
        let num = ctx.accounts.storage.num;
        let rotated = num.rotate_left((ctx.accounts.user.key().to_bytes()[0] % 32) as u32);
        ctx.accounts.storage.num = rotated;
        msg!("Case 128: rotated {} -> {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx128<'info> {
    #[account(mut, has_one = owner)]
    pub storage: Account<'info, Storage128>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Storage128 {
    pub owner: Pubkey,
    pub num: u64,
}
