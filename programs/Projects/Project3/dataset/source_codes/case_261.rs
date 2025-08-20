use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf261mvTWf");

#[program]
pub mod link_config_261 {
    use super::*;

    pub fn link_config(ctx: Context<Ctx261>, amount: u64) -> Result<()> {
        let base = ctx.accounts.record.value;
        let updated = base.checked_add(amount).unwrap();
        ctx.accounts.record.value = updated;
        msg!("Case 261: value {} → {}", base, updated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx261<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record261>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Record261 {
    pub matched: Pubkey,
    pub value: u64,
}
