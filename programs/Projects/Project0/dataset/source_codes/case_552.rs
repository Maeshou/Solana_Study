use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf552mvTWf");

#[program]
pub mod rotate_item_552 {
    use super::*;

    pub fn rotate_item(ctx: Context<Ctx552>, shift: u64) -> Result<()> {
        let prev = ctx.accounts.record.value;
        let rotated = prev.rotate_right((shift % 64) as u32);
        ctx.accounts.record.value = rotated;
        msg!("Case 552: value {} -> {}", prev, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx552<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record552>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record552 {
    pub owner: Pubkey,
    pub value: u64,
}
