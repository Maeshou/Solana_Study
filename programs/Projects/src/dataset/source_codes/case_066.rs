// Case 66: 支払い条件設定
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u066eKfp");

#[program]
pub mod case_066 {
    use super::*;

// Case 66: 支払い条件設定
pub fn execute_case_066(ctx: Context<ContextCase066>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_066;
    let recipient = &mut ctx.accounts.acct_dst_066;
    // Missing signer: signer_066
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase066<'info> {
    #[account(mut)]
    pub acct_src_066: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 支払い条件設定
    pub signer_066: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_066: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
