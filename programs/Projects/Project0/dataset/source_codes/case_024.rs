use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf024mvTWf");

#[program]
pub mod upgrade_configuration_024 {
    use super::*;

    pub fn upgrade_configuration(ctx: Context<Ctx024>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 024 by {}", ctx.accounts.user.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 024: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx024<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item024>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Item024 {
    pub owner: Pubkey,
    pub text: String,
}
