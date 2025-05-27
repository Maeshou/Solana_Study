// Case 20: プログラムアップグレード
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u020eKfp");

#[program]
pub mod case_020 {
    use super::*;

// Case 20: プログラムアップグレード
pub fn execute_case_020(ctx: Context<ContextCase020>, mint_amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let to = &ctx.accounts.acct_dst_020;
    // Missing signer on mint_authority: signer_020
    token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
    msg!("Minted {} tokens", mint_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase020<'info> {
    #[account(mut)]
    pub acct_src_020: Account<'info, TokenAccount>,
    /// CHECK: signer missing for プログラムアップグレード
    pub signer_020: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_020: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
