use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf072mvTWf");

#[program]
pub mod configure_registry_072 {
    use super::*;

    pub fn configure_registry(ctx: Context<Ctx072>) -> Result<()> {
        let previous = ctx.accounts.record.registry;
        let new_key = ctx.accounts.new_registry.key();
        ctx.accounts.record.registry = new_key;
        msg!("Case 072: registry updated from {} to {}", previous, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx072<'info> {
    #[account(mut, has_one = manager)]
    pub record: Account<'info, Record072>,
    #[account(signer)]
    pub manager: Signer<'info>,
    pub new_registry: Signer<'info>,
}

#[account]
pub struct Record072 {
    pub manager: Pubkey,
    pub registry: Pubkey,
}
