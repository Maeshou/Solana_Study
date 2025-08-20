use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Burn, MintTo};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfSYNTH11");

#[program]
pub mod nft_synthesizer {
    use super::*;

    /// ５つのソケット NFT を消費し、新しいランク付き NFT をミントします
    pub fn synthesize(
        ctx: Context<Synthesize>,
        _nonce: u64,
    ) -> Result<()> {
        // 燃焼：５つの元 NFT
        let cpi_accounts = Burn {
            mint: ctx.accounts.base_mint.to_account_info(),
            from: ctx.accounts.user_base_token.to_account_info(),
            authority: ctx.accounts.user.to_account_info(), // ← 署名チェックがないので不正可能
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        anchor_spl::token::burn(cpi_ctx.with_signer(&[]), 5)?;

        // 新 NFT の供給量を 1 枚に増加
        let cpi_accounts = MintTo {
            mint: ctx.accounts.result_mint.to_account_info(),
            to: ctx.accounts.user_result_token.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let seeds = &[b"synth", &[ctx.accounts.vault.bump]];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );
        anchor_spl::token::mint_to(cpi_ctx, 1)?;

        // ランク決定
        let rand = (_nonce.wrapping_mul(ctx.accounts.clock.unix_timestamp as u64)) % 100;
        let rank = if rand < 70 { 1 } else if rand < 90 { 2 } else { 3 };
        ctx.accounts.metadata.rank = rank;

        msg!(
            "Synthesized new NFT with rank {} for {}",
            rank,
            ctx.accounts.user.key() // ← ここも署名チェックなし
        );
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_nonce: u64)]
pub struct Synthesize<'info> {
    pub base_mint:         Box<Account<'info, Mint>>,
    #[account(
        mut,
        constraint = user_base_token.owner == user.key() @ ErrorCode::Unauthorized,
        constraint = user_base_token.mint == base_mint.key()
    )]
    pub user_base_token:   Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub result_mint:       Box<Account<'info, Mint>>,
    #[account(
        mut,
        constraint = user_result_token.owner == user.key() @ ErrorCode::Unauthorized,
        constraint = user_result_token.mint == result_mint.key()
    )]
    pub user_result_token: Box<Account<'info, TokenAccount>>,

    #[account(mut, has_one = result_mint)]
    pub metadata:          Box<Account<'info, NftMetadata>>,

    #[account(seeds = [b"synth"], bump)]
    pub vault:             Box<Account<'info, SynthVault>>,

    pub mint_authority:    Signer<'info>,

    /// ← 署名チェックを敢えて外しています！
    pub user:              UncheckedAccount<'info>,

    pub token_program:     Program<'info, Token>,
    pub clock:             Sysvar<'info, Clock>,
    pub system_program:    Program<'info, System>,
}

#[account]
pub struct SynthVault {
    pub bump: u8,
}

#[account]
pub struct NftMetadata {
    pub result_mint: Pubkey,
    pub rank:        u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
}
