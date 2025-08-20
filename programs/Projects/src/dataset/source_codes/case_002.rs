// Case 2: Mint（新規発行）機能
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u002eKfp");

#[program]
pub mod case_002 {
    use super::*;

// Case 2: Mint（新規発行）機能
pub fn execute_case_002(ctx: Context<ContextCase002>, mint_amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let to = &ctx.accounts.acct_dst_002;
    // Missing signer on mint_authority: signer_002
    token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
    msg!("Minted {} tokens", mint_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase002<'info> {
    #[account(mut)]
    pub acct_src_002: Account<'info, TokenAccount>,
    /// CHECK: signer missing for Mint（新規発行）機能
    pub signer_002: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_002: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
