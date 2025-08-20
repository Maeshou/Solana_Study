use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf039mvTWf");

#[program]
pub mod sync_delegate_039 {
    use super::*;

    pub fn sync_delegate(ctx: Context<Ctx039>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 039 by {}", ctx.accounts.user.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 039: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx039<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item039>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Item039 {
    pub owner: Pubkey,
    pub text: String,
}
