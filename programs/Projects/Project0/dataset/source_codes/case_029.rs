use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf029mvTWf");

#[program]
pub mod grant_registry_entry_029 {
    use super::*;

    pub fn grant_registry_entry(ctx: Context<Ctx029>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 029 by {}", ctx.accounts.user.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 029: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx029<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item029>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Item029 {
    pub owner: Pubkey,
    pub text: String,
}
