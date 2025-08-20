use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf092mvTWf");

#[program]
pub mod configure_registry_092 {
    use super::*;

    pub fn configure_registry(ctx: Context<Ctx092>) -> Result<()> {
        let previous = ctx.accounts.record.registry;
        let new_key = ctx.accounts.new_registry.key();
        ctx.accounts.record.registry = new_key;
        msg!("Case 092: registry updated from {} to {}", previous, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx092<'info> {
    #[account(mut, has_one = manager)]
    pub record: Account<'info, Record092>,
    #[account(signer)]
    pub manager: Signer<'info>,
    pub new_registry: Signer<'info>,
}

#[account]
pub struct Record092 {
    pub manager: Pubkey,
    pub registry: Pubkey,
}
