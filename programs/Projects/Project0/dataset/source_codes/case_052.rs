use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf052mvTWf");

#[program]
pub mod configure_registry_052 {
    use super::*;

    pub fn configure_registry(ctx: Context<Ctx052>) -> Result<()> {
        let previous = ctx.accounts.record.registry;
        let new_key = ctx.accounts.new_registry.key();
        ctx.accounts.record.registry = new_key;
        msg!("Case 052: registry updated from {} to {}", previous, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx052<'info> {
    #[account(mut, has_one = manager)]
    pub record: Account<'info, Record052>,
    #[account(signer)]
    pub manager: Signer<'info>,
    pub new_registry: Signer<'info>,
}

#[account]
pub struct Record052 {
    pub manager: Pubkey,
    pub registry: Pubkey,
}
