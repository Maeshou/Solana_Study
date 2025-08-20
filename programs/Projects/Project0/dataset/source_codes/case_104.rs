use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf104mvTWf");

#[program]
pub mod authorize_account_104 {
    use super::*;

    pub fn authorize_account(ctx: Context<Ctx104>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 104 executed by {}", ctx.accounts.actor.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 104: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx104<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item104>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item104 {
    pub owner: Pubkey,
    pub text: String,
}
