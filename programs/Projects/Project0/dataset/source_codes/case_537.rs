use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf537mvTWf");

#[program]
pub mod calibrate_value_537 {
    use super::*;

    pub fn calibrate_value(ctx: Context<Ctx537>, shift: u64) -> Result<()> {
        let prev = ctx.accounts.record.value;
        let rotated = prev.rotate_right((shift % 64) as u32);
        ctx.accounts.record.value = rotated;
        msg!("Case 537: value {} -> {}", prev, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx537<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record537>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record537 {
    pub owner: Pubkey,
    pub value: u64,
}
