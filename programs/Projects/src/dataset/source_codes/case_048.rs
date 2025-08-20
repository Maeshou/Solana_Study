// Case 48: 資金プール破棄
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u048eKfp");

#[program]
pub mod case_048 {
    use super::*;

// Case 48: 資金プール破棄
pub fn execute_case_048(ctx: Context<ContextCase048>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.acct_src_048;
    let recipient = &mut ctx.accounts.acct_dst_048;
    // Missing signer: signer_048
    treasury.distribute(recipient.key(), amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase048<'info> {
    #[account(mut)]
    pub acct_src_048: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 資金プール破棄
    pub signer_048: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_048: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
