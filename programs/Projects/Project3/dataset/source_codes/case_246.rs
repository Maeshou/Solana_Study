use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf246mvTWf");

#[program]
pub mod sync_ledger_246 {
    use super::*;

    pub fn sync_ledger(ctx: Context<Ctx246>, amount: u64) -> Result<()> {
        let base = ctx.accounts.record.value;
        let updated = base.checked_add(amount).unwrap();
        ctx.accounts.record.value = updated;
        msg!("Case 246: value {} → {}", base, updated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx246<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record246>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Record246 {
    pub matched: Pubkey,
    pub value: u64,
}
