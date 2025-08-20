use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf266mvTWf");

#[program]
pub mod sync_ledger_266 {
    use super::*;

    pub fn sync_ledger(ctx: Context<Ctx266>, amount: u64) -> Result<()> {
        let base = ctx.accounts.record.value;
        let updated = base.checked_add(amount).unwrap();
        ctx.accounts.record.value = updated;
        msg!("Case 266: value {} → {}", base, updated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx266<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record266>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Record266 {
    pub matched: Pubkey,
    pub value: u64,
}
