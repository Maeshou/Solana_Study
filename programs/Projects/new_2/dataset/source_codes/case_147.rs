use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, MintTo, Token};

declare_id!("VulnMint3333333333333333333333333333333333");

#[program]
pub mod vuln_mint {
    pub fn mint_to_user(ctx: Context<MintCtx>, amount: u64) -> Result<()> {
        // authority が正当かどうか全く検証せずにミント
        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint:      ctx.accounts.mint.to_account_info(),
                to:        ctx.accounts.dest.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        anchor_spl::token::mint_to(cpi, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintCtx<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub dest: Account<'info, anchor_spl::token::TokenAccount>,
    /// CHECK: authority の検証なし
    pub authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
