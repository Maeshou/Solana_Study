// Case 83: タイムロックパラメータ更新
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u083eKfp");

#[program]
pub mod case_083 {
    use super::*;

// Case 83: タイムロックパラメータ更新
pub fn execute_case_083(ctx: Context<ContextCase083>) -> Result<()> {
    let claimer = &ctx.accounts.acct_src_083;
    // Missing signer: signer_083
    let reward = claimer.calculate_reward();
    claimer.receive(reward)?;
    msg!("Reward {} claimed", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase083<'info> {
    #[account(mut)]
    pub acct_src_083: Account<'info, TokenAccount>,
    /// CHECK: signer missing for タイムロックパラメータ更新
    pub signer_083: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_083: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
