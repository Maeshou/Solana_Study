use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0851ED6ID");

#[program]
pub mod reward_distribution_case_085 {
    use super::*;

    pub fn reward_distribution_action_085(ctx: Context<Reward_distributionCtx085>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.reward_distribution_src_085.to_account_info(), to: ctx.accounts.reward_distribution_dst_085.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"reward_distribution", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Reward_distributionCtx085<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub reward_distribution_src_085: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub reward_distribution_dst_085: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"reward_distribution"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



