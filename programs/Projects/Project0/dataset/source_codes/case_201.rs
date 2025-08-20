use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf201mvTWf");

#[program]
pub mod link_config_201 {
    use super::*;

    pub fn link_config(ctx: Context<Ctx201>, amount: u64) -> Result<()> {
        let base = ctx.accounts.record.value;
        let updated = base.checked_add(amount).unwrap();
        ctx.accounts.record.value = updated;
        msg!("Case 201: value {} → {}", base, updated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx201<'info> {
    #[account(mut, has_one = matched)]
    pub record: Account<'info, Record201>,
    #[account(signer)]
    pub matched: Signer<'info>,
}

#[account]
pub struct Record201 {
    pub matched: Pubkey,
    pub value: u64,
}
