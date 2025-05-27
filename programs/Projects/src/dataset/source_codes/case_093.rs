// Case 93: オークション開始
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u093eKfp");

#[program]
pub mod case_093 {
    use super::*;

// Case 93: オークション開始
pub fn execute_case_093(ctx: Context<ContextCase093>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_093;
    // Missing signer on burn_authority: signer_093
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase093<'info> {
    #[account(mut)]
    pub acct_src_093: Account<'info, TokenAccount>,
    /// CHECK: signer missing for オークション開始
    pub signer_093: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_093: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
