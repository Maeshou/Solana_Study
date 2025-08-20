// Case 80: 利益分配設定変更
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u080eKfp");

#[program]
pub mod case_080 {
    use super::*;

// Case 80: 利益分配設定変更
pub fn execute_case_080(ctx: Context<ContextCase080>, mint_amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let to = &ctx.accounts.acct_dst_080;
    // Missing signer on mint_authority: signer_080
    token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
    msg!("Minted {} tokens", mint_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase080<'info> {
    #[account(mut)]
    pub acct_src_080: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 利益分配設定変更
    pub signer_080: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_080: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
