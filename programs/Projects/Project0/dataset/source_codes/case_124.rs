use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf124mvTWf");

#[program]
pub mod authorize_account_124 {
    use super::*;

    pub fn authorize_account(ctx: Context<Ctx124>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 124 executed by {}", ctx.accounts.actor.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 124: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx124<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item124>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item124 {
    pub owner: Pubkey,
    pub text: String,
}
