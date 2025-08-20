use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0011E91ID");

#[program]
pub mod subscription_case_001 {
    use super::*;

    pub fn subscription_action_001(ctx: Context<SubscriptionCtx001>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.subscription_src_001.to_account_info(), to: ctx.accounts.subscription_dst_001.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"subscription", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubscriptionCtx001<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub subscription_src_001: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub subscription_dst_001: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"subscription"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



