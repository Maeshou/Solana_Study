// Case 59: アラート設定
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u059eKfp");

#[program]
pub mod case_059 {
    use super::*;

// Case 59: アラート設定
pub fn execute_case_059(ctx: Context<ContextCase059>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_059;
    // Missing signer: signer_059
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase059<'info> {
    #[account(mut)]
    pub acct_src_059: Account<'info, TokenAccount>,
    /// CHECK: signer missing for アラート設定
    pub signer_059: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_059: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
