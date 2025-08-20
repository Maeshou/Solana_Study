// Case 36: 任意アドレスへの資金送金
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u036eKfp");

#[program]
pub mod case_036 {
    use super::*;

// Case 36: 任意アドレスへの資金送金
pub fn execute_case_036(ctx: Context<ContextCase036>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_036;
    let recipient = &mut ctx.accounts.acct_dst_036;
    // Missing signer: signer_036
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase036<'info> {
    #[account(mut)]
    pub acct_src_036: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 任意アドレスへの資金送金
    pub signer_036: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_036: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
