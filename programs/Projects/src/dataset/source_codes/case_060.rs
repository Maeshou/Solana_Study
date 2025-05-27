// Case 60: アラート解除
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u060eKfp");

#[program]
pub mod case_060 {
    use super::*;

// Case 60: アラート解除
pub fn execute_case_060(ctx: Context<ContextCase060>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_060;
    let recipient = &mut ctx.accounts.acct_dst_060;
    // Missing signer: signer_060
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase060<'info> {
    #[account(mut)]
    pub acct_src_060: Account<'info, TokenAccount>,
    /// CHECK: signer missing for アラート解除
    pub signer_060: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_060: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
