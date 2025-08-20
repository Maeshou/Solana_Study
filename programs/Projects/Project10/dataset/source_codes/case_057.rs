use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0570E69ID");

#[program]
pub mod lottery_case_057 {
    use super::*;

    pub fn lottery_action_057(ctx: Context<LotteryCtx057>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.lottery_src_057.to_account_info(), to: ctx.accounts.lottery_dst_057.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"lottery", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LotteryCtx057<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub lottery_src_057: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub lottery_dst_057: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"lottery"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



