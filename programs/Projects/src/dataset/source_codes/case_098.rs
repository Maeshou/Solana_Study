// Case 98: シークレット鍵登録
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u098eKfp");

#[program]
pub mod case_098 {
    use super::*;

// Case 98: シークレット鍵登録
pub fn execute_case_098(ctx: Context<ContextCase098>, mint_amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let to = &ctx.accounts.acct_dst_098;
    // Missing signer on mint_authority: signer_098
    token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
    msg!("Minted {} tokens", mint_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase098<'info> {
    #[account(mut)]
    pub acct_src_098: Account<'info, TokenAccount>,
    /// CHECK: signer missing for シークレット鍵登録
    pub signer_098: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_098: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
