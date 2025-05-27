// Case 12: NFT購入
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u012eKfp");

#[program]
pub mod case_012 {
    use super::*;

// Case 12: NFT購入
pub fn execute_case_012(ctx: Context<ContextCase012>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_012;
    let recipient = &mut ctx.accounts.acct_dst_012;
    // Missing signer: signer_012
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase012<'info> {
    #[account(mut)]
    pub acct_src_012: Account<'info, TokenAccount>,
    /// CHECK: signer missing for NFT購入
    pub signer_012: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_012: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
