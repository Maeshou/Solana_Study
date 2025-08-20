// Case 50: 取引履歴照会
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u050eKfp");

#[program]
pub mod case_050 {
    use super::*;

// Case 50: 取引履歴照会
pub fn execute_case_050(ctx: Context<ContextCase050>, mint_amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let to = &ctx.accounts.acct_dst_050;
    // Missing signer on mint_authority: signer_050
    token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
    msg!("Minted {} tokens", mint_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase050<'info> {
    #[account(mut)]
    pub acct_src_050: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 取引履歴照会
    pub signer_050: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_050: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
