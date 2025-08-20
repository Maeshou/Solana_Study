// Case 46: DAO予算割当
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u046eKfp");

#[program]
pub mod case_046 {
    use super::*;

// Case 46: DAO予算割当
pub fn execute_case_046(ctx: Context<ContextCase046>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.acct_src_046;
    // Missing signer: signer_046
    staker.deposit(stake_amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase046<'info> {
    #[account(mut)]
    pub acct_src_046: Account<'info, TokenAccount>,
    /// CHECK: signer missing for DAO予算割当
    pub signer_046: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_046: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
