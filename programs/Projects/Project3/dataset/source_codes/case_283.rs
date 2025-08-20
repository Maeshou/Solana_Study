use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf283mvTWf");

#[program]
pub mod link_entry_283 {
    use super::*;

    pub fn link_entry(ctx: Context<Ctx283>, shift: u64) -> Result<()> {
        let num = ctx.accounts.storage.counter;
        let rotated = num.rotate_right((shift % 64) as u32);
        ctx.accounts.storage.counter = rotated;
        msg!("Case 283: rotated {} → {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx283<'info> {
    #[account(mut, has_one = matched)]
    pub storage: Account<'info, Storage283>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Storage283 {
    pub matched: Pubkey,
    pub counter: u64,
}
