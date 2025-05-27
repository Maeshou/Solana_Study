// Case 17: マルチシグ提案承認
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u017eKfp");

#[program]
pub mod case_017 {
    use super::*;

// Case 17: マルチシグ提案承認
pub fn execute_case_017(ctx: Context<ContextCase017>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_017;
    // Missing signer: signer_017
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase017<'info> {
    #[account(mut)]
    pub acct_src_017: Account<'info, TokenAccount>,
    /// CHECK: signer missing for マルチシグ提案承認
    pub signer_017: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_017: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
