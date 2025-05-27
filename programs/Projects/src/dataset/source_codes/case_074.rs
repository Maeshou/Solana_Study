// Case 74: オーダーキャンセル
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u074eKfp");

#[program]
pub mod case_074 {
    use super::*;

// Case 74: オーダーキャンセル
pub fn execute_case_074(ctx: Context<ContextCase074>, mint_amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let to = &ctx.accounts.acct_dst_074;
    // Missing signer on mint_authority: signer_074
    token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
    msg!("Minted {} tokens", mint_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase074<'info> {
    #[account(mut)]
    pub acct_src_074: Account<'info, TokenAccount>,
    /// CHECK: signer missing for オーダーキャンセル
    pub signer_074: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_074: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
