use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf238mvTWf");

#[program]
pub mod sync_state_238 {
    use super::*;

    pub fn sync_state(ctx: Context<Ctx238>, shift: u64) -> Result<()> {
        let num = ctx.accounts.storage.counter;
        let rotated = num.rotate_right((shift % 64) as u32);
        ctx.accounts.storage.counter = rotated;
        msg!("Case 238: rotated {} → {}", num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx238<'info> {
    #[account(mut, has_one = matched)]
    pub storage: Account<'info, Storage238>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Storage238 {
    pub matched: Pubkey,
    pub counter: u64,
}
