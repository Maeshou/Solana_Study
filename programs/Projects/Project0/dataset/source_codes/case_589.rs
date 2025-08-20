use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf589mvTWf");

#[program]
pub mod update_setting_589 {
    use super::*;

    pub fn update_setting(ctx: Context<Ctx589>, info_str: String) -> Result<()> {
        let old_text = ctx.accounts.record.text.clone();
        ctx.accounts.record.text = info_str.clone();
        msg!("Case 589: '{}' -> '{}'", old_text, info_str);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx589<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record589>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record589 {
    pub owner: Pubkey,
    pub text: String,
}
