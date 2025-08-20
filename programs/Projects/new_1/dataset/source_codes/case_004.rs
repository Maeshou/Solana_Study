use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf004mvTWf");

#[program]
pub mod upgrade_configuration_004 {
    use super::*;

    pub fn upgrade_configuration(ctx: Context<Ctx004>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 004 by {}", ctx.accounts.user.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 004: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx004<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item004>,
    pub owner: AccountInfo<'info>,
    pub user: AccountInfo<'info>,
}

#[account]
pub struct Item004 {
    pub owner: Pubkey,
    pub text: String,
}
