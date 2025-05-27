// Case 56: ガバナンス委任取消
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u056eKfp");

#[program]
pub mod case_056 {
    use super::*;

// Case 56: ガバナンス委任取消
pub fn execute_case_056(ctx: Context<ContextCase056>, mint_amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let to = &ctx.accounts.acct_dst_056;
    // Missing signer on mint_authority: signer_056
    token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
    msg!("Minted {} tokens", mint_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase056<'info> {
    #[account(mut)]
    pub acct_src_056: Account<'info, TokenAccount>,
    /// CHECK: signer missing for ガバナンス委任取消
    pub signer_056: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_056: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
