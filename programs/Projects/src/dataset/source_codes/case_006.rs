// Case 6: 報酬請求（Reward Claim）
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u006eKfp");

#[program]
pub mod case_006 {
    use super::*;

// Case 6: 報酬請求（Reward Claim）
pub fn execute_case_006(ctx: Context<ContextCase006>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_006;
    let recipient = &mut ctx.accounts.acct_dst_006;
    // Missing signer: signer_006
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase006<'info> {
    #[account(mut)]
    pub acct_src_006: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 報酬請求（Reward Claim）
    pub signer_006: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_006: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
