
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct StakeFarmCtxuwor<'info> {
    #[account(mut)] pub farm: Account<'info, DataAccount>,
    #[account(mut)] pub farmer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_048 {
    use super::*;

    pub fn stake_farm(ctx: Context<StakeFarmCtxuwor>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.farm;
        // custom logic for stake_farm
        assert!(ctx.accounts.farm.data > 0); acct.data -= amount;
        msg!("Executed stake_farm logic");
        Ok(())
    }
}
