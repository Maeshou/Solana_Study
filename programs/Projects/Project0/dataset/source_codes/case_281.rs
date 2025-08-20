use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf281mvTWf");

#[program]
pub mod link_config_281 {
    use super::*;

    pub fn link_config(ctx: Context<Ctx281>, amount: u64) -> Result<()> {
        let base = ctx.accounts.record.value;
        let updated = base.checked_add(amount).unwrap();
        ctx.accounts.record.value = updated;
        msg!("Case 281: value {} → {}", base, updated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx281<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record281>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Record281 {
    pub matched: Pubkey,
    pub value: u64,
}
