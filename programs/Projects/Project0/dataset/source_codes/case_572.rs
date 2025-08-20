use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf572mvTWf");

#[program]
pub mod rotate_item_572 {
    use super::*;

    pub fn rotate_item(ctx: Context<Ctx572>, shift: u64) -> Result<()> {
        let prev = ctx.accounts.record.value;
        let rotated = prev.rotate_right((shift % 64) as u32);
        ctx.accounts.record.value = rotated;
        msg!("Case 572: value {} -> {}", prev, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx572<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record572>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record572 {
    pub owner: Pubkey,
    pub value: u64,
}
