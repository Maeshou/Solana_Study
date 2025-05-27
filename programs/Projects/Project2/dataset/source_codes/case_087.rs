
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct OpenPositionCtxrihw<'info> {
    #[account(mut)] pub position: Account<'info, DataAccount>,
    #[account(mut)] pub trader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_087 {
    use super::*;

    pub fn open_position(ctx: Context<OpenPositionCtxrihw>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.position;
        // custom logic for open_position
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed open_position logic");
        Ok(())
    }
}
