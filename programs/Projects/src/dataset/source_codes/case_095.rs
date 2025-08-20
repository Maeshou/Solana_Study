// Case 95: オークション終了
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u095eKfp");

#[program]
pub mod case_095 {
    use super::*;

// Case 95: オークション終了
pub fn execute_case_095(ctx: Context<ContextCase095>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_095;
    // Missing signer: signer_095
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase095<'info> {
    #[account(mut)]
    pub acct_src_095: Account<'info, TokenAccount>,
    /// CHECK: signer missing for オークション終了
    pub signer_095: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_095: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
