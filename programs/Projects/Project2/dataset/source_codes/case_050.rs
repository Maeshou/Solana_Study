
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct HarvestCtxjsdb<'info> {
    #[account(mut)] pub farm: Account<'info, DataAccount>,
    #[account(mut)] pub farmer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_050 {
    use super::*;

    pub fn harvest(ctx: Context<HarvestCtxjsdb>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.farm;
        // custom logic for harvest
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed harvest logic");
        Ok(())
    }
}
