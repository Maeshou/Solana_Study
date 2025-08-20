use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf507mvTWf");

#[program]
pub mod calibrate_value_507 {
    use super::*;

    pub fn calibrate_value(ctx: Context<Ctx507>, shift: u64) -> Result<()> {
        let prev = ctx.accounts.record.value;
        let rotated = prev.rotate_right((shift % 64) as u32);
        ctx.accounts.record.value = rotated;
        msg!("Case 507: value {} -> {}", prev, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx507<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record507>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record507 {
    pub owner: Pubkey,
    pub value: u64,
}
