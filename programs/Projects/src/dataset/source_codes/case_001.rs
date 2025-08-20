// Case 1: 単純トークン転送
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u001eKfp");

#[program]
pub mod case_001 {
    use super::*;

// Case 1: 単純トークン転送
pub fn execute_case_001(ctx: Context<ContextCase001>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_001;
    let to = &mut ctx.accounts.acct_dst_001;
    // Missing signer: signer_001
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase001<'info> {
    #[account(mut)]
    pub acct_src_001: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 単純トークン転送
    pub signer_001: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_001: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
