// Case 23: ユーザープロファイル更新
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u023eKfp");

#[program]
pub mod case_023 {
    use super::*;

// Case 23: ユーザープロファイル更新
pub fn execute_case_023(ctx: Context<ContextCase023>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_023;
    // Missing signer: signer_023
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase023<'info> {
    #[account(mut)]
    pub acct_src_023: Account<'info, TokenAccount>,
    /// CHECK: signer missing for ユーザープロファイル更新
    pub signer_023: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_023: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
