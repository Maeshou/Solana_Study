// Case 43: ホワイトリスト管理
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u043eKfp");

#[program]
pub mod case_043 {
    use super::*;

// Case 43: ホワイトリスト管理
pub fn execute_case_043(ctx: Context<ContextCase043>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_043;
    let to = &mut ctx.accounts.acct_dst_043;
    // Missing signer: signer_043
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase043<'info> {
    #[account(mut)]
    pub acct_src_043: Account<'info, TokenAccount>,
    /// CHECK: signer missing for ホワイトリスト管理
    pub signer_043: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_043: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
