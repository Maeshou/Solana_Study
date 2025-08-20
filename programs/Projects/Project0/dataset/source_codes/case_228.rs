use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf228mvTWf");

#[program]
pub mod sync_state_228 {
    use super::*;

    pub fn sync_state(ctx: Context<Ctx228>, shift: u64) -> Result<()> {
        let num = ctx.accounts.storage.counter;
        let rotated = num.rotate_right((shift % 64) as u32);
        ctx.accounts.storage.counter = rotated;
        msg!("Case 228: rotated {} → {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx228<'info> {
    #[account(mut, has_one = matched)]
    pub storage: Account<'info, Storage228>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Storage228 {
    pub matched: Pubkey,
    pub counter: u64,
}
