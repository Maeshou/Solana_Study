use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf253mvTWf");

#[program]
pub mod link_entry_253 {
    use super::*;

    pub fn link_entry(ctx: Context<Ctx253>, shift: u64) -> Result<()> {
        let num = ctx.accounts.storage.counter;
        let rotated = num.rotate_right((shift % 64) as u32);
        ctx.accounts.storage.counter = rotated;
        msg!("Case 253: rotated {} → {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx253<'info> {
    #[account(mut, has_one = matched)]
    pub storage: Account<'info, Storage253>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Storage253 {
    pub matched: Pubkey,
    pub counter: u64,
}
