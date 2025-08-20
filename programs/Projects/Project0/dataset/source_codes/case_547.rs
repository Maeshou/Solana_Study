use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf547mvTWf");

#[program]
pub mod calibrate_value_547 {
    use super::*;

    pub fn calibrate_value(ctx: Context<Ctx547>, shift: u64) -> Result<()> {
        let prev = ctx.accounts.record.value;
        let rotated = prev.rotate_right((shift % 64) as u32);
        ctx.accounts.record.value = rotated;
        msg!("Case 547: value {} -> {}", prev, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx547<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record547>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record547 {
    pub owner: Pubkey,
    pub value: u64,
}
