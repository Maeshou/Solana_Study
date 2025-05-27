// Case 86: プロフィール写真変更
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u086eKfp");

#[program]
pub mod case_086 {
    use super::*;

// Case 86: プロフィール写真変更
pub fn execute_case_086(ctx: Context<ContextCase086>, mint_amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let to = &ctx.accounts.acct_dst_086;
    // Missing signer on mint_authority: signer_086
    token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
    msg!("Minted {} tokens", mint_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase086<'info> {
    #[account(mut)]
    pub acct_src_086: Account<'info, TokenAccount>,
    /// CHECK: signer missing for プロフィール写真変更
    pub signer_086: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_086: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
