use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf059mvTWf");

#[program]
pub mod sync_delegate_059 {
    use super::*;

    pub fn sync_delegate(ctx: Context<Ctx059>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 059 by {}", ctx.accounts.user.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 059: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx059<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item059>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Item059 {
    pub owner: Pubkey,
    pub text: String,
}
