use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf044mvTWf");

#[program]
pub mod upgrade_configuration_044 {
    use super::*;

    pub fn upgrade_configuration(ctx: Context<Ctx044>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 044 by {}", ctx.accounts.user.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 044: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx044<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item044>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Item044 {
    pub owner: Pubkey,
    pub text: String,
}
