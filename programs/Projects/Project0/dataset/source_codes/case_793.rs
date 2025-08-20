use anchor_lang::prelude::*;
use anchor_spl::associated_token::create as ata_create;
use anchor_spl::token::{mint_to, MintTo, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf793mvTWf");

#[program]
pub mod build_and_mint_793 {
    use super::*;

    pub fn build_and_mint(ctx: Context<BuildAndMint793>, amount: u64) -> Result<()> {
        let ata_bump = *ctx.bumps.get("ata_state").unwrap();
        ata_create(ctx.accounts.into());
        let cpi = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.ata.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        mint_to(CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi), amount)?;
        let st = &mut ctx.accounts.ata_state;
        st.bump = ata_bump;
        st.minted = st.minted.checked_add(amount).unwrap();
        msg!("Case 793: ata_bump={} minted_total={}", ata_bump, st.minted);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuildAndMint793<'info> {
    #[account(address = anchor_spl::associated_token::ID)]
    pub ata_program: Program<'info, anchor_spl::token::Token>,
    #[account(init, associated_token::mint = mint, associated_token::authority = payer)]
    pub ata: Account<'info, anchor_spl::token::TokenAccount>,
    #[account(mut)] pub payer: Signer<'info>,
    pub mint: Account<'info, anchor_spl::token::Mint>,
    #[account(init, seeds = [b"ata_state"], bump, payer = payer, space = 8 + 1 + 8)]
    pub ata_state: Account<'info, AtaState793>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct AtaState793 {
    pub bump: u8,
    pub minted: u64,
}
