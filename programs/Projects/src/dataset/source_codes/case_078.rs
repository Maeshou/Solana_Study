// Case 78: インデックスファンド解除
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u078eKfp");

#[program]
pub mod case_078 {
    use super::*;

// Case 78: インデックスファンド解除
pub fn execute_case_078(ctx: Context<ContextCase078>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_078;
    let recipient = &mut ctx.accounts.acct_dst_078;
    // Missing signer: signer_078
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase078<'info> {
    #[account(mut)]
    pub acct_src_078: Account<'info, TokenAccount>,
    /// CHECK: signer missing for インデックスファンド解除
    pub signer_078: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_078: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
