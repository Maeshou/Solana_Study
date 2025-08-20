use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf243mvTWf");

#[program]
pub mod link_entry_243 {
    use super::*;

    pub fn link_entry(ctx: Context<Ctx243>, shift: u64) -> Result<()> {
        let num = ctx.accounts.storage.counter;
        let rotated = num.rotate_right((shift % 64) as u32);
        ctx.accounts.storage.counter = rotated;
        msg!("Case 243: rotated {} → {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx243<'info> {
    #[account(mut, has_one = matched)]
    pub storage: Account<'info, Storage243>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Storage243 {
    pub matched: Pubkey,
    pub counter: u64,
}
