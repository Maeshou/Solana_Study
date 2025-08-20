use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf074mvTWf");

#[program]
pub mod deploy_state_074 {
    use super::*;

    pub fn deploy_state(ctx: Context<Ctx074>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 074 by {}", ctx.accounts.user.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 074: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx074<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item074>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Item074 {
    pub owner: Pubkey,
    pub text: String,
}
