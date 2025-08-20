use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf203mvTWf");

#[program]
pub mod link_entry_203 {
    use super::*;

    pub fn link_entry(ctx: Context<Ctx203>, shift: u64) -> Result<()> {
        let num = ctx.accounts.storage.counter;
        let rotated = num.rotate_right((shift % 64) as u32);
        ctx.accounts.storage.counter = rotated;
        msg!("Case 203: rotated {} → {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx203<'info> {
    #[account(mut, has_one = matched)]
    pub storage: Account<'info, Storage203>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Storage203 {
    pub matched: Pubkey,
    pub counter: u64,
}
