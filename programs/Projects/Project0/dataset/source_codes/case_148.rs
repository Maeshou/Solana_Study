use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf148mvTWf");

#[program]
pub mod refresh_account_148 {
    use super::*;

    pub fn refresh_account(ctx: Context<Ctx148>) -> Result<()> {
        let num = ctx.accounts.storage.num;
        let rotated = num.rotate_left((ctx.accounts.user.key().to_bytes()[0] % 32) as u32);
        ctx.accounts.storage.num = rotated;
        msg!("Case 148: rotated {} -> {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx148<'info> {
    #[account(mut, has_one = owner)]
    pub storage: Account<'info, Storage148>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Storage148 {
    pub owner: Pubkey,
    pub num: u64,
}
