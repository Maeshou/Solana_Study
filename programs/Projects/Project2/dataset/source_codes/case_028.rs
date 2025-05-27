
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct BurnNftCtxtiga<'info> {
    #[account(mut)] pub metadata: Account<'info, DataAccount>,
    #[account(mut)] pub owner: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_028 {
    use super::*;

    pub fn burn_nft(ctx: Context<BurnNftCtxtiga>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.metadata;
        // custom logic for burn_nft
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed burn_nft logic");
        Ok(())
    }
}
