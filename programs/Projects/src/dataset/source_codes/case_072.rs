// Case 72: マーケットオーダー発注
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u072eKfp");

#[program]
pub mod case_072 {
    use super::*;

// Case 72: マーケットオーダー発注
pub fn execute_case_072(ctx: Context<ContextCase072>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_072;
    let recipient = &mut ctx.accounts.acct_dst_072;
    // Missing signer: signer_072
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase072<'info> {
    #[account(mut)]
    pub acct_src_072: Account<'info, TokenAccount>,
    /// CHECK: signer missing for マーケットオーダー発注
    pub signer_072: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_072: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
