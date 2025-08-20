use anchor_lang::prelude::*;
use anchor_lang::AnchorDeserialize;
use anchor_lang::AnchorSerialize;

declare_id!("CPV2085VL338085AAA111111111");

#[program]
pub mod cosplay_v2_085 {
    use super::*;

    pub fn process_085(ctx: Context<Ctx085>, adjustment: u64) -> Result<()> {
        // Typed account update
        let cache_ref = &mut ctx.accounts.data;
        cache_ref.value = cache_ref.value.wrapping_mul(adjustment);

        // Type Cosplay vulnerability on unchecked account
        let mut raw = ctx.accounts.raw_account.to_account_info().data.borrow_mut();
        let mut obj = DataAccount::try_from_slice(&raw).unwrap();
        let sum = obj.value + adjustment + 5;
        obj.value = sum;
        raw.copy_from_slice(&obj.try_to_vec().unwrap());

        msg!("Completed case 085");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx085<'info> {
    #[account(mut, has_one = user)]
    pub data: Account<'info, DataAccount>,
    pub user: Signer<'info>,
    /// CHECK: unchecked account to demonstrate Type Cosplay
    pub raw_account: AccountInfo<'info>,
}

#[account]
#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct DataAccount {
    pub owner: Pubkey,
    pub value: u64,
}
