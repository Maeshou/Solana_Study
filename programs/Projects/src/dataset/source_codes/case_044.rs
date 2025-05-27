// Case 44: トークンバーンレート変更
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u044eKfp");

#[program]
pub mod case_044 {
    use super::*;

// Case 44: トークンバーンレート変更
pub fn execute_case_044(ctx: Context<ContextCase044>, mint_amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let to = &ctx.accounts.acct_dst_044;
    // Missing signer on mint_authority: signer_044
    token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
    msg!("Minted {} tokens", mint_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase044<'info> {
    #[account(mut)]
    pub acct_src_044: Account<'info, TokenAccount>,
    /// CHECK: signer missing for トークンバーンレート変更
    pub signer_044: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_044: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
