// Case 65: IPFSハッシュ設定
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u065eKfp");

#[program]
pub mod case_065 {
    use super::*;

// Case 65: IPFSハッシュ設定
pub fn execute_case_065(ctx: Context<ContextCase065>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_065;
    // Missing signer: signer_065
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase065<'info> {
    #[account(mut)]
    pub acct_src_065: Account<'info, TokenAccount>,
    /// CHECK: signer missing for IPFSハッシュ設定
    pub signer_065: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_065: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
