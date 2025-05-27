
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct StartArbCtxalmu<'info> {
    #[account(mut)] pub arb_manager: Account<'info, DataAccount>,
    #[account(mut)] pub trader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_085 {
    use super::*;

    pub fn start_arb(ctx: Context<StartArbCtxalmu>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.arb_manager;
        // custom logic for start_arb
        **ctx.accounts.arb_manager.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed start_arb logic");
        Ok(())
    }
}
