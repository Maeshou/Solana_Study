use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf211mvTWf");

#[program]
pub mod link_config_211 {
    use super::*;

    pub fn link_config(ctx: Context<Ctx211>, amount: u64) -> Result<()> {
        let base = ctx.accounts.record.value;
        let updated = base.checked_add(amount).unwrap();
        ctx.accounts.record.value = updated;
        msg!("Case 211: value {} → {}", base, updated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx211<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record211>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Record211 {
    pub matched: Pubkey,
    pub value: u64,
}
