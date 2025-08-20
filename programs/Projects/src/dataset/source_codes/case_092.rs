// Case 92: 担保評価更新
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u092eKfp");

#[program]
pub mod case_092 {
    use super::*;

// Case 92: 担保評価更新
pub fn execute_case_092(ctx: Context<ContextCase092>, mint_amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let to = &ctx.accounts.acct_dst_092;
    // Missing signer on mint_authority: signer_092
    token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
    msg!("Minted {} tokens", mint_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase092<'info> {
    #[account(mut)]
    pub acct_src_092: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 担保評価更新
    pub signer_092: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_092: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
