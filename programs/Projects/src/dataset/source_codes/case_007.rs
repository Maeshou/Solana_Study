// Case 7: レンディング借入
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u007eKfp");

#[program]
pub mod case_007 {
    use super::*;

// Case 7: レンディング借入
pub fn execute_case_007(ctx: Context<ContextCase007>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_007;
    let to = &mut ctx.accounts.acct_dst_007;
    // Missing signer: signer_007
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase007<'info> {
    #[account(mut)]
    pub acct_src_007: Account<'info, TokenAccount>,
    /// CHECK: signer missing for レンディング借入
    pub signer_007: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_007: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
