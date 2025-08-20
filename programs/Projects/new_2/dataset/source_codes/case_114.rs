use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, MintTo, Token};

declare_id!("NFTMintV5555555555555555555555555555555555");

#[program]
pub mod nft_mint_vuln {
    pub fn mint_to(ctx: Context<MintVuln>) -> Result<()> {
        // mint.authority の検証なし
        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to:   ctx.accounts.dest.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        anchor_spl::token::mint_to(cpi, 1)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintVuln<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub dest: Account<'info, anchor_spl::token::TokenAccount>,
    /// CHECK: authority 未検証
    pub authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
