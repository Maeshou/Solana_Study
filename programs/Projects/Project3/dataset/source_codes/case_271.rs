use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf271mvTWf");

#[program]
pub mod link_config_271 {
    use super::*;

    pub fn link_config(ctx: Context<Ctx271>, amount: u64) -> Result<()> {
        let base = ctx.accounts.record.value;
        let updated = base.checked_add(amount).unwrap();
        ctx.accounts.record.value = updated;
        msg!("Case 271: value {} → {}", base, updated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx271<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record271>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Record271 {
    pub matched: Pubkey,
    pub value: u64,
}
