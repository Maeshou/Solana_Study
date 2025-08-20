use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, MintTo};

declare_id!("AtaCreate88888888888888888888888888888888");

#[program]
pub mod ata_creator {
    use super::*;

    pub fn build_ata(ctx: Context<BuildAta>) -> Result<()> {
        // ATA を作成
        let cpi_create = CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: ctx.accounts.payer.to_account_info(),
                associated_token: ctx.accounts.ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );
        anchor_spl::associated_token::create(cpi_create)?;

        // トークンを ATA にミント
        let cpi_mint = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        anchor_spl::token::mint_to(cpi_mint, ctx.accounts.initial_amount)?;
        emit!(AtaBuilt {
            user: ctx.accounts.user.key(),
            amount: ctx.accounts.initial_amount
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuildAta<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// ATA は AssociatedToken で導き出される
    pub ata: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[event]
pub struct AtaBuilt {
    pub user: Pubkey,
    pub amount: u64,
}
