use anchor_lang::prelude::*;
declare_id!("ESCR0031111111111111111111111111111111111111");

#[program]
pub mod case003 {
    use super::*;
    pub fn execute_escrow(ctx: Context<EscrowContext>) -> Result<()> {
        // Place lamports into escrow for a duration
        let escrow_amount = 200u64;
        **ctx.accounts.account_a.to_account_info().lamports.borrow_mut() -= escrow_amount;
        msg!("Escrowed {} lamports", escrow_amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EscrowContext<'info> {
    /// CHECK: expecting EscrowAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting EscrowAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EscrowAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}