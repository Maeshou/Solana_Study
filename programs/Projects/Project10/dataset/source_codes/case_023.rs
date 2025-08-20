use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH02308E3ID");

#[program]
pub mod governance_case_023 {
    use super::*;

    pub fn governance_action_023(ctx: Context<GovernanceCtx023>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.governance_dst_023.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"governance", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.governance_dst_023.to_account_info(), to: ctx.accounts.governance_src_023.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"governance", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GovernanceCtx023<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub governance_dst_023: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub governance_src_023: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"governance"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



