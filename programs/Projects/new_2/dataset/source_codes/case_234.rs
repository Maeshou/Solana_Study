use anchor_lang::prelude::*;
use anchor_spl::token::{MintTo, TokenAccount, Token};

declare_id!("VulnEx33000000000000000000000000000000000033");

#[program]
pub mod example33 {
    pub fn mint_nft(ctx: Context<Ctx33>, amount: u64) -> Result<()> {
        // proof_log は所有者検証なし
        ctx.accounts.proof_log.data.borrow_mut().extend_from_slice(&[amount as u8]);
        // mint_account は has_one で authority 検証済み
        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint:      ctx.accounts.mint.to_account_info(),
                to:        ctx.accounts.mint_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        anchor_spl::token::mint_to(cpi, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx33<'info> {
    #[account(mut)]
    pub proof_log: AccountInfo<'info>,
    #[account(mut, has_one = authority)]
    pub mint_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub mint: Account<'info, anchor_spl::token::Mint>,
    pub token_program: Program<'info, Token>,
}

