use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf009mvTWf");

#[program]
pub mod grant_registry_entry_009 {
    use super::*;

    pub fn grant_registry_entry(ctx: Context<Ctx009>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 009 by {}", ctx.accounts.user.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 009: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx009<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item009>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Item009 {
    pub owner: Pubkey,
    pub text: String,
}
