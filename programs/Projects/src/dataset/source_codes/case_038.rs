// Case 38: 支払いチャネル開設
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u038eKfp");

#[program]
pub mod case_038 {
    use super::*;

// Case 38: 支払いチャネル開設
pub fn execute_case_038(ctx: Context<ContextCase038>, mint_amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let to = &ctx.accounts.acct_dst_038;
    // Missing signer on mint_authority: signer_038
    token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
    msg!("Minted {} tokens", mint_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase038<'info> {
    #[account(mut)]
    pub acct_src_038: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 支払いチャネル開設
    pub signer_038: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_038: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
