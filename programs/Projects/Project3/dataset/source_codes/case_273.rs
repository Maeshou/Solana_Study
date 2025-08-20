use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf273mvTWf");

#[program]
pub mod link_entry_273 {
    use super::*;

    pub fn link_entry(ctx: Context<Ctx273>, shift: u64) -> Result<()> {
        let num = ctx.accounts.storage.counter;
        let rotated = num.rotate_right((shift % 64) as u32);
        ctx.accounts.storage.counter = rotated;
        msg!("Case 273: rotated {} → {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx273<'info> {
    #[account(mut, has_one = matched)]
    pub storage: Account<'info, Storage273>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Storage273 {
    pub matched: Pubkey,
    pub counter: u64,
}
