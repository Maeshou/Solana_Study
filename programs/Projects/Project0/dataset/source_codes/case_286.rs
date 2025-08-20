use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf286mvTWf");

#[program]
pub mod sync_ledger_286 {
    use super::*;

    pub fn sync_ledger(ctx: Context<Ctx286>, amount: u64) -> Result<()> {
        let base = ctx.accounts.record.value;
        let updated = base.checked_add(amount).unwrap();
        ctx.accounts.record.value = updated;
        msg!("Case 286: value {} → {}", base, updated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx286<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record286>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Record286 {
    pub matched: Pubkey,
    pub value: u64,
}
