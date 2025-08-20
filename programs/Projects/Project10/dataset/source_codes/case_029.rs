use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH02919A2ID");

#[program]
pub mod liquidation_case_029 {
    use super::*;

    pub fn liquidation_action_029(ctx: Context<LiquidationCtx029>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.liquidation_src_029.to_account_info(), to: ctx.accounts.liquidation_dst_029.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"liquidation", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LiquidationCtx029<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub liquidation_src_029: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub liquidation_dst_029: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"liquidation"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



