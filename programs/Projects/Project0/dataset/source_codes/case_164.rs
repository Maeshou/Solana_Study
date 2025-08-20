use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf164mvTWf");

#[program]
pub mod authorize_account_164 {
    use super::*;

    pub fn authorize_account(ctx: Context<Ctx164>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 164 executed by {}", ctx.accounts.actor.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 164: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx164<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item164>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item164 {
    pub owner: Pubkey,
    pub text: String,
}
