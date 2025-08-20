// Case 32: 借入清算（Liquidation）
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u032eKfp");

#[program]
pub mod case_032 {
    use super::*;

// Case 32: 借入清算（Liquidation）
pub fn execute_case_032(ctx: Context<ContextCase032>, mint_amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let to = &ctx.accounts.acct_dst_032;
    // Missing signer on mint_authority: signer_032
    token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
    msg!("Minted {} tokens", mint_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase032<'info> {
    #[account(mut)]
    pub acct_src_032: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 借入清算（Liquidation）
    pub signer_032: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_032: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
