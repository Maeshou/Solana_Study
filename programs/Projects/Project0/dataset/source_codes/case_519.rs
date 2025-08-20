use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf519mvTWf");

#[program]
pub mod update_setting_519 {
    use super::*;

    pub fn update_setting(ctx: Context<Ctx519>, info_str: String) -> Result<()> {
        let old_text = ctx.accounts.record.text.clone();
        ctx.accounts.record.text = info_str.clone();
        msg!("Case 519: '{}' -> '{}'", old_text, info_str);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx519<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record519>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record519 {
    pub owner: Pubkey,
    pub text: String,
}
