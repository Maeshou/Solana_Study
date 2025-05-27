
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClosePositionCtxdoyw<'info> {
    #[account(mut)] pub position: Account<'info, DataAccount>,
    #[account(mut)] pub trader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_088 {
    use super::*;

    pub fn close_position(ctx: Context<ClosePositionCtxdoyw>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.position;
        // custom logic for close_position
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed close_position logic");
        Ok(())
    }
}
