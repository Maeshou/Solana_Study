// Case 90: 資産担保登録
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u090eKfp");

#[program]
pub mod case_090 {
    use super::*;

// Case 90: 資産担保登録
pub fn execute_case_090(ctx: Context<ContextCase090>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_090;
    let recipient = &mut ctx.accounts.acct_dst_090;
    // Missing signer: signer_090
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase090<'info> {
    #[account(mut)]
    pub acct_src_090: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 資産担保登録
    pub signer_090: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_090: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
