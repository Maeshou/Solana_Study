// Case 27: 報酬配布
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u027eKfp");

#[program]
pub mod case_027 {
    use super::*;

// Case 27: 報酬配布
pub fn execute_case_027(ctx: Context<ContextCase027>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_027;
    // Missing signer on burn_authority: signer_027
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase027<'info> {
    #[account(mut)]
    pub acct_src_027: Account<'info, TokenAccount>,
    /// CHECK: signer missing for 報酬配布
    pub signer_027: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_027: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
