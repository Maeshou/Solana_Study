// Case 13: エスクロー資金解除
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u013eKfp");

#[program]
pub mod case_013 {
    use super::*;

// Case 13: エスクロー資金解除
pub fn execute_case_013(ctx: Context<ContextCase013>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_013;
    let to = &mut ctx.accounts.acct_dst_013;
    // Missing signer: signer_013
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase013<'info> {
    #[account(mut)]
    pub acct_src_013: Account<'info, TokenAccount>,
    /// CHECK: signer missing for エスクロー資金解除
    pub signer_013: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_013: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
