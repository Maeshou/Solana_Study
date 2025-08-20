// Case 47: 資金プール生成
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u047eKfp");

#[program]
pub mod case_047 {
    use super::*;

// Case 47: 資金プール生成
pub fn execute_case_047(ctx: Context<ContextCase047>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_047;
    // Missing signer: signer_047
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase047<'info> {
    #[account(mut)]
    pub acct_src_047: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 資金プール生成
    pub signer_047: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_047: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
