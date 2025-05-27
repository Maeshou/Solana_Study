// Case 4: ステーキング入金
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u004eKfp");

#[program]
pub mod case_004 {
    use super::*;

// Case 4: ステーキング入金
pub fn execute_case_004(ctx: Context<ContextCase004>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_004;
    // Missing signer: signer_004
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase004<'info> {
    #[account(mut)]
    pub acct_src_004: Account<'info, TokenAccount>,
    /// CHECK: signer missing for ステーキング入金
    pub signer_004: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_004: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
