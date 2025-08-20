use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf206mvTWf");

#[program]
pub mod sync_ledger_206 {
    use super::*;

    pub fn sync_ledger(ctx: Context<Ctx206>, amount: u64) -> Result<()> {
        let base = ctx.accounts.record.value;
        let updated = base.checked_add(amount).unwrap();
        ctx.accounts.record.value = updated;
        msg!("Case 206: value {} → {}", base, updated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx206<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record206>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Record206 {
    pub matched: Pubkey,
    pub value: u64,
}
