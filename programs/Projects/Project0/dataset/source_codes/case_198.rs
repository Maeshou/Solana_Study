use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf198mvTWf");

#[program]
pub mod refresh_account_198 {
    use super::*;

    pub fn refresh_account(ctx: Context<Ctx198>) -> Result<()> {
        let num = ctx.accounts.storage.num;
        let rotated = num.rotate_left((ctx.accounts.user.key().to_bytes()[0] % 32) as u32);
        ctx.accounts.storage.num = rotated;
        msg!("Case 198: rotated {} -> {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx198<'info> {
    #[account(mut, has_one = owner)]
    pub storage: Account<'info, Storage198>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Storage198 {
    pub owner: Pubkey,
    pub num: u64,
}
