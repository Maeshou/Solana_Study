use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0712064ID");

#[program]
pub mod governance_case_071 {
    use super::*;

    pub fn governance_action_071(ctx: Context<GovernanceCtx071>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.governance_src_071.to_account_info(), to: ctx.accounts.governance_dst_071.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"governance", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GovernanceCtx071<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub governance_src_071: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub governance_dst_071: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"governance"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



