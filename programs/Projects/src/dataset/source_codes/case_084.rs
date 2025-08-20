// Case 84: スナップショット取得
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u084eKfp");

#[program]
pub mod case_084 {
    use super::*;

// Case 84: スナップショット取得
pub fn execute_case_084(ctx: Context<ContextCase084>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_084;
    let recipient = &mut ctx.accounts.acct_dst_084;
    // Missing signer: signer_084
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase084<'info> {
    #[account(mut)]
    pub acct_src_084: Account<'info, TokenAccount>,
    /// CHECK: signer missing for スナップショット取得
    pub signer_084: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_084: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
