use anchor_lang::prelude::*;
use anchor_lang::AnchorDeserialize;
use anchor_lang::AnchorSerialize;

declare_id!("TCOSPLAY520XXXXXXXXXXXXXXX");

#[program]
pub mod safe_cosplay_520 {
    use super::*;

    pub fn modify_value(ctx: Context<Modify520>, delta: u64) -> Result<()> {
        // Properly typed account
        let acct = &mut ctx.accounts.typed_acc;
        acct.value = acct.value.checked_add(delta).unwrap();

        // Raw account is also checked for type
        let raw_acc: Account<DataAcc> = Account::try_from(&ctx.accounts.raw_acc)?;
        raw_acc.value = raw_acc.value.checked_sub(delta).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Modify520<'info> {
    #[account(mut)]
    pub typed_acc: Account<'info, DataAcc>,
    #[account(mut)]
    pub raw_acc: Account<'info, DataAcc>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct DataAcc {
    pub value: u64,
}