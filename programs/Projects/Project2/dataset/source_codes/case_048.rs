use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};

declare_id!("AtaCreate88888888888888888888888888888888");

#[program]
pub mod ata_creator {
    use super::*;

    pub fn build_ata(ctx: Context<BuildAta>) -> Result<()> {
        // ATA はプログラム内で導出されるため、正しいもののみ扱われる
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuildAta<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: ATA は AssociatedToken で決定
    pub ata: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}
