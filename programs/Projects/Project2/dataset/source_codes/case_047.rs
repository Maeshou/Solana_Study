
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateFarmCtxhcre<'info> {
    #[account(mut)] pub farm: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_047 {
    use super::*;

    pub fn create_farm(ctx: Context<CreateFarmCtxhcre>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.farm;
        // custom logic for create_farm
        assert!(ctx.accounts.farm.data > 0); acct.data -= amount;
        msg!("Executed create_farm logic");
        Ok(())
    }
}
