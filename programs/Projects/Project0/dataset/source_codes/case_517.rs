use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf517mvTWf");

#[program]
pub mod calibrate_value_517 {
    use super::*;

    pub fn calibrate_value(ctx: Context<Ctx517>, shift: u64) -> Result<()> {
        let prev = ctx.accounts.record.value;
        let rotated = prev.rotate_right((shift % 64) as u32);
        ctx.accounts.record.value = rotated;
        msg!("Case 517: value {} -> {}", prev, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx517<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record517>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record517 {
    pub owner: Pubkey,
    pub value: u64,
}
