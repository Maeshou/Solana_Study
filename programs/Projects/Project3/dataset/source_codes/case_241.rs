use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf241mvTWf");

#[program]
pub mod link_config_241 {
    use super::*;

    pub fn link_config(ctx: Context<Ctx241>, amount: u64) -> Result<()> {
        let base = ctx.accounts.record.value;
        let updated = base.checked_add(amount).unwrap();
        ctx.accounts.record.value = updated;
        msg!("Case 241: value {} → {}", base, updated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx241<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record241>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Record241 {
    pub matched: Pubkey,
    pub value: u64,
}
