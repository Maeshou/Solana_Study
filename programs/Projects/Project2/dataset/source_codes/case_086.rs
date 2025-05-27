
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EndArbCtxposx<'info> {
    #[account(mut)] pub arb_manager: Account<'info, DataAccount>,
    #[account(mut)] pub trader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_086 {
    use super::*;

    pub fn end_arb(ctx: Context<EndArbCtxposx>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.arb_manager;
        // custom logic for end_arb
        **ctx.accounts.arb_manager.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed end_arb logic");
        Ok(())
    }
}
