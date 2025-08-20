// Case 54: ペナルティ適用
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u054eKfp");

#[program]
pub mod case_054 {
    use super::*;

// Case 54: ペナルティ適用
pub fn execute_case_054(ctx: Context<ContextCase054>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_054;
    let recipient = &mut ctx.accounts.acct_dst_054;
    // Missing signer: signer_054
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase054<'info> {
    #[account(mut)]
    pub acct_src_054: Account<'info, TokenAccount>,
    /// CHECK: signer missing for ペナルティ適用
    pub signer_054: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_054: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
