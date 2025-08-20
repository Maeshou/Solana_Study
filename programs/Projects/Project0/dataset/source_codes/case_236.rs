use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf236mvTWf");

#[program]
pub mod sync_ledger_236 {
    use super::*;

    pub fn sync_ledger(ctx: Context<Ctx236>, amount: u64) -> Result<()> {
        let base = ctx.accounts.record.value;
        let updated = base.checked_add(amount).unwrap();
        ctx.accounts.record.value = updated;
        msg!("Case 236: value {} → {}", base, updated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx236<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record236>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Record236 {
    pub matched: Pubkey,
    pub value: u64,
}
