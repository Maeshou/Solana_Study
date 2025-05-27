
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AddMarginCtxader<'info> {
    #[account(mut)] pub position: Account<'info, DataAccount>,
    #[account(mut)] pub trader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_089 {
    use super::*;

    pub fn add_margin(ctx: Context<AddMarginCtxader>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.position;
        // custom logic for add_margin
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed add_margin logic");
        Ok(())
    }
}
