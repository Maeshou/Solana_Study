// Case 15: DEX流動性追加
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u015eKfp");

#[program]
pub mod case_015 {
    use super::*;

// Case 15: DEX流動性追加
pub fn execute_case_015(ctx: Context<ContextCase015>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_015;
    // Missing signer on burn_authority: signer_015
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase015<'info> {
    #[account(mut)]
    pub acct_src_015: Account<'info, TokenAccount>,
    /// CHECK: signer missing for DEX流動性追加
    pub signer_015: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_015: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
