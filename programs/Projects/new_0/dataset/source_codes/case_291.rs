use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf291mvTWf");

#[program]
pub mod link_config_291 {
    use super::*;

    pub fn link_config(ctx: Context<Ctx291>, amount: u64) -> Result<()> {
        let base = ctx.accounts.record.value;
        let updated = base.checked_add(amount).unwrap();
        ctx.accounts.record.value = updated;
        msg!("Case 291: value {} → {}", base, updated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx291<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record291>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Record291 {
    pub matched: Pubkey,
    pub value: u64,
}
