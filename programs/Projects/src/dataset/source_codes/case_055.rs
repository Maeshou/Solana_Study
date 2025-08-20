// Case 55: ガバナンス委任設定
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token, mint_to, burn
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg3b2u055eKfp");

#[program]
pub mod case_055 {
    use super::*;

// Case 55: ガバナンス委任設定
pub fn execute_case_055(ctx: Context<ContextCase055>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.acct_src_055;
    let to = &mut ctx.accounts.acct_dst_055;
    // Missing signer: signer_055
    **from.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;
    msg!("Transferred {} lamports", amount);
    Ok(())
}

}

#[derive(Accounts)]
pub struct ContextCase055<'info> {
    #[account(mut)]
    pub acct_src_055: Account<'info, TokenAccount>,
    /// CHECK: signer missing for ガバナンス委任設定
    pub signer_055: AccountInfo<'info>,
    #[account(mut)]
    pub acct_dst_055: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
