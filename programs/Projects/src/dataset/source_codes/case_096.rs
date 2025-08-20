// Case 96: オークション取消
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u096eKfp");

#[program]
pub mod case_096 {
    use super::*;

// Case 96: オークション取消
pub fn execute_case_096(ctx: Context<ContextCase096>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_096;
    let recipient = &mut ctx.accounts.acct_dst_096;
    // Missing signer: signer_096
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase096<'info> {
    #[account(mut)]
    pub acct_src_096: Account<'info, TokenAccount>,
    /// CHECK: signer missing for オークション取消
    pub signer_096: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_096: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
