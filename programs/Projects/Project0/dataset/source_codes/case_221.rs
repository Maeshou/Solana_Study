use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf221mvTWf");

#[program]
pub mod link_config_221 {
    use super::*;

    pub fn link_config(ctx: Context<Ctx221>, amount: u64) -> Result<()> {
        let base = ctx.accounts.record.value;
        let updated = base.checked_add(amount).unwrap();
        ctx.accounts.record.value = updated;
        msg!("Case 221: value {} → {}", base, updated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx221<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record221>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Record221 {
    pub matched: Pubkey,
    pub value: u64,
}
