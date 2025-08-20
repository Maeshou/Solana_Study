use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf542mvTWf");

#[program]
pub mod rotate_item_542 {
    use super::*;

    pub fn rotate_item(ctx: Context<Ctx542>, shift: u64) -> Result<()> {
        let prev = ctx.accounts.record.value;
        let rotated = prev.rotate_right((shift % 64) as u32);
        ctx.accounts.record.value = rotated;
        msg!("Case 542: value {} -> {}", prev, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx542<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record542>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record542 {
    pub owner: Pubkey,
    pub value: u64,
}
