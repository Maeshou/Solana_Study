use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf562mvTWf");

#[program]
pub mod rotate_item_562 {
    use super::*;

    pub fn rotate_item(ctx: Context<Ctx562>, shift: u64) -> Result<()> {
        let prev = ctx.accounts.record.value;
        let rotated = prev.rotate_right((shift % 64) as u32);
        ctx.accounts.record.value = rotated;
        msg!("Case 562: value {} -> {}", prev, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx562<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record562>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record562 {
    pub owner: Pubkey,
    pub value: u64,
}
