use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf069mvTWf");

#[program]
pub mod grant_registry_entry_069 {
    use super::*;

    pub fn grant_registry_entry(ctx: Context<Ctx069>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 069 by {}", ctx.accounts.user.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 069: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx069<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item069>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Item069 {
    pub owner: Pubkey,
    pub text: String,
}
