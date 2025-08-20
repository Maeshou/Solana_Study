use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf114mvTWf");

#[program]
pub mod authorize_account_114 {
    use super::*;

    pub fn authorize_account(ctx: Context<Ctx114>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 114 executed by {}", ctx.accounts.actor.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 114: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx114<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item114>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item114 {
    pub owner: Pubkey,
    pub text: String,
}
