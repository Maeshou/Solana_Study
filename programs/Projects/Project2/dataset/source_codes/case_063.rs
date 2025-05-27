
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct BuyTicketCtxoacm<'info> {
    #[account(mut)] pub lottery: Account<'info, DataAccount>,
    #[account(mut)] pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_063 {
    use super::*;

    pub fn buy_ticket(ctx: Context<BuyTicketCtxoacm>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.lottery;
        // custom logic for buy_ticket
        **ctx.accounts.lottery.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed buy_ticket logic");
        Ok(())
    }
}
