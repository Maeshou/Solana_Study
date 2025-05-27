// Case 8: レンディング返済
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u008eKfp");

#[program]
pub mod case_008 {
    use super::*;

// Case 8: レンディング返済
pub fn execute_case_008(ctx: Context<ContextCase008>, mint_amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let to = &ctx.accounts.acct_dst_008;
    // Missing signer on mint_authority: signer_008
    token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
    msg!("Minted {} tokens", mint_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase008<'info> {
    #[account(mut)]
    pub acct_src_008: Account<'info, TokenAccount>,
    /// CHECK: signer missing for レンディング返済
    pub signer_008: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_008: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
