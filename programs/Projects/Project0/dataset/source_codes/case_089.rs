use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf089mvTWf");

#[program]
pub mod grant_registry_entry_089 {
    use super::*;

    pub fn grant_registry_entry(ctx: Context<Ctx089>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 089 by {}", ctx.accounts.user.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 089: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx089<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item089>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Item089 {
    pub owner: Pubkey,
    pub text: String,
}
