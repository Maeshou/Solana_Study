use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf522mvTWf");

#[program]
pub mod rotate_item_522 {
    use super::*;

    pub fn rotate_item(ctx: Context<Ctx522>, shift: u64) -> Result<()> {
        let prev = ctx.accounts.record.value;
        let rotated = prev.rotate_right((shift % 64) as u32);
        ctx.accounts.record.value = rotated;
        msg!("Case 522: value {} -> {}", prev, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx522<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record522>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record522 {
    pub owner: Pubkey,
    pub value: u64,
}
