use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, MintTo, Token};

declare_id!("VulnMnt3333333333333333333333333333333333");

#[program]
pub mod vuln_mint {
    pub fn mint(ctx: Context<MintOne>) -> Result<()> {
        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.dest.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        // authority が本当に mint.authority か検証していない
        anchor_spl::token::mint_to(cpi, 1)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintOne<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub dest: Account<'info, TokenAccount>,
    /// CHECK: authority 検証なし
    pub authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
