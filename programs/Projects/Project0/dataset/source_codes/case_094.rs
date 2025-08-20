use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf094mvTWf");

#[program]
pub mod deploy_state_094 {
    use super::*;

    pub fn deploy_state(ctx: Context<Ctx094>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 094 by {}", ctx.accounts.user.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 094: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx094<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item094>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Item094 {
    pub owner: Pubkey,
    pub text: String,
}
