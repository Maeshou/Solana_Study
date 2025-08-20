// Case 14: DEXスワップ
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u014eKfp");

#[program]
pub mod case_014 {
    use super::*;

// Case 14: DEXスワップ
pub fn execute_case_014(ctx: Context<ContextCase014>, mint_amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let to = &ctx.accounts.acct_dst_014;
    // Missing signer on mint_authority: signer_014
    token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
    msg!("Minted {} tokens", mint_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase014<'info> {
    #[account(mut)]
    pub acct_src_014: Account<'info, TokenAccount>,
    /// CHECK: signer missing for DEXスワップ
    pub signer_014: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_014: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
