use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf288mvTWf");

#[program]
pub mod sync_state_288 {
    use super::*;

    pub fn sync_state(ctx: Context<Ctx288>, shift: u64) -> Result<()> {
        let num = ctx.accounts.storage.counter;
        let rotated = num.rotate_right((shift % 64) as u32);
        ctx.accounts.storage.counter = rotated;
        msg!("Case 288: rotated {} → {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx288<'info> {
    #[account(mut, has_one = matched)]
    pub storage: Account<'info, Storage288>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Storage288 {
    pub matched: Pubkey,
    pub counter: u64,
}
