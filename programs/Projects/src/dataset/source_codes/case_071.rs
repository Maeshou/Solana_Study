// Case 71: クーポン利用
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u071eKfp");

#[program]
pub mod case_071 {
    use super::*;

// Case 71: クーポン利用
pub fn execute_case_071(ctx: Context<ContextCase071>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_071;
    // Missing signer: signer_071
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase071<'info> {
    #[account(mut)]
    pub acct_src_071: Account<'info, TokenAccount>,
    /// CHECK: signer missing for クーポン利用
    pub signer_071: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_071: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
