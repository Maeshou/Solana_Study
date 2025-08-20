use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf194mvTWf");

#[program]
pub mod authorize_account_194 {
    use super::*;

    pub fn authorize_account(ctx: Context<Ctx194>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 194 executed by {}", ctx.accounts.actor.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 194: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx194<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item194>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item194 {
    pub owner: Pubkey,
    pub text: String,
}
