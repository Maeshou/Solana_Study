
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UnstakeFarmCtxiuaj<'info> {
    #[account(mut)] pub farm: Account<'info, DataAccount>,
    #[account(mut)] pub farmer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_049 {
    use super::*;

    pub fn unstake_farm(ctx: Context<UnstakeFarmCtxiuaj>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.farm;
        // custom logic for unstake_farm
        **ctx.accounts.farm.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed unstake_farm logic");
        Ok(())
    }
}
