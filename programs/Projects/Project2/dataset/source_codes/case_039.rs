
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateEscrowCtxxeft<'info> {
    #[account(mut)] pub escrow: Account<'info, DataAccount>,
    #[account(mut)] pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_039 {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrowCtxxeft>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.escrow;
        // custom logic for create_escrow
        **ctx.accounts.escrow.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed create_escrow logic");
        Ok(())
    }
}
