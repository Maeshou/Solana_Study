// Case 53: 利息計算パラメータ更新
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u053eKfp");

#[program]
pub mod case_053 {
    use super::*;

// Case 53: 利息計算パラメータ更新
pub fn execute_case_053(ctx: Context<ContextCase053>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_053;
    // Missing signer: signer_053
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase053<'info> {
    #[account(mut)]
    pub acct_src_053: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 利息計算パラメータ更新
    pub signer_053: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_053: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
