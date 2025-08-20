// Case 41: PDA再割当
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u041eKfp");

#[program]
pub mod case_041 {
    use super::*;

// Case 41: PDA再割当
pub fn execute_case_041(ctx: Context<ContextCase041>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_041;
    // Missing signer: signer_041
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase041<'info> {
    #[account(mut)]
    pub acct_src_041: Account<'info, TokenAccount>,
    /// CHECK: signer missing for PDA再割当
    pub signer_041: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_041: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
