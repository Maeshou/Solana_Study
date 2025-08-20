use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf226mvTWf");

#[program]
pub mod sync_ledger_226 {
    use super::*;

    pub fn sync_ledger(ctx: Context<Ctx226>, amount: u64) -> Result<()> {
        let base = ctx.accounts.record.value;
        let updated = base.checked_add(amount).unwrap();
        ctx.accounts.record.value = updated;
        msg!("Case 226: value {} → {}", base, updated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx226<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record226>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Record226 {
    pub matched: Pubkey,
    pub value: u64,
}
