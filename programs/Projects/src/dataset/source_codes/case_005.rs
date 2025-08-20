// Case 5: ステーキング出金
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u005eKfp");

#[program]
pub mod case_005 {
    use super::*;

// Case 5: ステーキング出金
pub fn execute_case_005(ctx: Context<ContextCase005>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_005;
    // Missing signer: signer_005
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase005<'info> {
    #[account(mut)]
    pub acct_src_005: Account<'info, TokenAccount>,
    /// CHECK: signer missing for ステーキング出金
    pub signer_005: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_005: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
