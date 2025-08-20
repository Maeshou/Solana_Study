use anchor_lang::prelude::*;
declare_id!("TRAN0011111111111111111111111111111111111111");

#[program]
pub mod case001 {
    use super::*;
    pub fn execute_transfer(ctx: Context<TransferContext>) -> Result<()> {
        // Transfer lamports from account_a to account_b
        let amount = 1000u64;
        let mut src = ctx.accounts.account_a.to_account_info();
        let mut dst = ctx.accounts.account_b.to_account_info();
        **src.lamports.borrow_mut() = src.lamports().checked_sub(amount).unwrap();
        **dst.lamports.borrow_mut() = dst.lamports().checked_add(amount).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferContext<'info> {
    /// CHECK: expecting TransferAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting TransferAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TransferAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}