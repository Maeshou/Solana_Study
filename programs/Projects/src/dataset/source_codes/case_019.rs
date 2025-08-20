// Case 19: CPI（Cross-Program Invocation）呼び出し
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u019eKfp");

#[program]
pub mod case_019 {
    use super::*;

// Case 19: CPI（Cross-Program Invocation）呼び出し
pub fn execute_case_019(ctx: Context<ContextCase019>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_019;
    let to = &mut ctx.accounts.acct_dst_019;
    // Missing signer: signer_019
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase019<'info> {
    #[account(mut)]
    pub acct_src_019: Account<'info, TokenAccount>,
    /// CHECK: signer missing for CPI（Cross-Program Invocation）呼び出し
    pub signer_019: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_019: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
