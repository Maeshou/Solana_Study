// Case 79: 利益分配
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u079eKfp");

#[program]
pub mod case_079 {
    use super::*;

// Case 79: 利益分配
pub fn execute_case_079(ctx: Context<ContextCase079>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_079;
    let to = &mut ctx.accounts.acct_dst_079;
    // Missing signer: signer_079
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase079<'info> {
    #[account(mut)]
    pub acct_src_079: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 利益分配
    pub signer_079: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_079: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
