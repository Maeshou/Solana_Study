// Case 9: ガバナンス提案作成
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u009eKfp");

#[program]
pub mod case_009 {
    use super::*;

// Case 9: ガバナンス提案作成
pub fn execute_case_009(ctx: Context<ContextCase009>, burn_amount: u64) -> Result<()> {
    let from = &ctx.accounts.acct_src_009;
    // Missing signer on burn_authority: signer_009
    token::burn(ctx.accounts.into_burn_context(), burn_amount)?;
    msg!("Burned {} tokens", burn_amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase009<'info> {
    #[account(mut)]
    pub acct_src_009: Account<'info, TokenAccount>,
    /// CHECK: signer missing for ガバナンス提案作成
    pub signer_009: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_009: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
