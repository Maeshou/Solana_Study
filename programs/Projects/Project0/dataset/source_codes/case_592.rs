use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf592mvTWf");

#[program]
pub mod rotate_item_592 {
    use super::*;

    pub fn rotate_item(ctx: Context<Ctx592>, shift: u64) -> Result<()> {
        let prev = ctx.accounts.record.value;
        let rotated = prev.rotate_right((shift % 64) as u32);
        ctx.accounts.record.value = rotated;
        msg!("Case 592: value {} -> {}", prev, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx592<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record592>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record592 {
    pub owner: Pubkey,
    pub value: u64,
}
