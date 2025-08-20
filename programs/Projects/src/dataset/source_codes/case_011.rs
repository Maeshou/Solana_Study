// Case 11: NFTマーケット出品
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u011eKfp");

#[program]
pub mod case_011 {
    use super::*;

// Case 11: NFTマーケット出品
pub fn execute_case_011(ctx: Context<ContextCase011>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_011;
    // Missing signer: signer_011
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase011<'info> {
    #[account(mut)]
    pub acct_src_011: Account<'info, TokenAccount>,
    /// CHECK: signer missing for NFTマーケット出品
    pub signer_011: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_011: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
