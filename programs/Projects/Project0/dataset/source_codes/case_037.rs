use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf037mvTWf");

#[program]
pub mod tune_setting_037 {
    use super::*;

    pub fn tune_setting(ctx: Context<Ctx037>) -> Result<()> {
        let previous = ctx.accounts.record.setting;
        let new_key = ctx.accounts.new_setting.key();
        ctx.accounts.record.setting = new_key;
        msg!("Case 037: setting updated from {} to {}", previous, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx037<'info> {
    #[account(mut, has_one = manager)]
    pub record: Account<'info, Record037>,
    #[account(signer)]
    pub manager: Signer<'info>,
    pub new_setting: Signer<'info>,
}

#[account]
pub struct Record037 {
    pub manager: Pubkey,
    pub setting: Pubkey,
}
