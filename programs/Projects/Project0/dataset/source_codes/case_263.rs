use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf263mvTWf");

#[program]
pub mod link_entry_263 {
    use super::*;

    pub fn link_entry(ctx: Context<Ctx263>, shift: u64) -> Result<()> {
        let num = ctx.accounts.storage.counter;
        let rotated = num.rotate_right((shift % 64) as u32);
        ctx.accounts.storage.counter = rotated;
        msg!("Case 263: rotated {} → {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx263<'info> {
    #[account(mut, has_one = matched)]
    pub storage: Account<'info, Storage263>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Storage263 {
    pub matched: Pubkey,
    pub counter: u64,
}
