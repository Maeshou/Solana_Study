// Case 81: ペイアウトスケジュール更新
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u081eKfp");

#[program]
pub mod case_081 {
    use super::*;

// Case 81: ペイアウトスケジュール更新
pub fn execute_case_081(ctx: Context<ContextCase081>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_081;
    // Missing signer on burn_authority: signer_081
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase081<'info> {
    #[account(mut)]
    pub acct_src_081: Account<'info, TokenAccount>,
    /// CHECK: signer missing for ペイアウトスケジュール更新
    pub signer_081: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_081: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
