// Case 30: クロスチェーンブリッジ実行
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u030eKfp");

#[program]
pub mod case_030 {
    use super::*;

// Case 30: クロスチェーンブリッジ実行
pub fn execute_case_030(ctx: Context<ContextCase030>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_030;
    let recipient = &mut ctx.accounts.acct_dst_030;
    // Missing signer: signer_030
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase030<'info> {
    #[account(mut)]
    pub acct_src_030: Account<'info, TokenAccount>,
    /// CHECK: signer missing for クロスチェーンブリッジ実行
    pub signer_030: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_030: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
