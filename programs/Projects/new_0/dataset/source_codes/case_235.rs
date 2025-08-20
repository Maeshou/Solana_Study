use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token, MintTo};

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpWHITELIST1234567890ABCDEF");

#[program]
pub mod whitelist_mint {
    use super::*;

    /// ホワイトリストアカウントの初期化
    pub fn initialize(
        ctx: Context<InitializeWhitelist>,
        bump: u8,
    ) -> ProgramResult {
        let wl = &mut ctx.accounts.whitelist;
        wl.authority = *ctx.accounts.authority.key;
        wl.mint = *ctx.accounts.mint.to_account_info().key;
        wl.bump = bump;
        wl.entries = Vec::new();
        Ok(())
    }

    /// ホワイトリストへのアドレス追加
    pub fn add_member(
        ctx: Context<ModifyWhitelist>,
        member: Pubkey,
    ) -> ProgramResult {
        let wl = &mut ctx.accounts.whitelist;
        wl.entries.push(member);
        Ok(())
    }

    /// ホワイトリストからアドレス削除
    pub fn remove_member(
        ctx: Context<ModifyWhitelist>,
        member: Pubkey,
    ) -> ProgramResult {
        let wl = &mut ctx.accounts.whitelist;
        wl.entries.retain(|x| x != &member);
        Ok(())
    }

    /// ホワイトリスト済みアドレスによるNFTミント
    pub fn mint_nft(
        ctx: Context<MintNft>,
    ) -> ProgramResult {
        let wl = &ctx.accounts.whitelist;
        let user_key = ctx.accounts.user.key();
        require!(wl.entries.contains(&user_key), WhitelistError::NotWhitelisted);

        // PDA as mint authority
        let seeds = &[b"whitelist", wl.mint.as_ref(), &[wl.bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.whitelist.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );
        mint_to(cpi_ctx, 1)?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeWhitelist<'info> {
    #[account(
        init,
        seeds = [b"whitelist", mint.key().as_ref()],
        bump = bump,
        payer = authority,
        space = 8 + 32 + 32 + 1 + 4 + 100 * 32,
    )]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// ミントをPDAの権限に設定済みであることを保証
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ModifyWhitelist<'info> {
    #[account(
        mut,
        seeds = [b"whitelist", whitelist.mint.as_ref()],
        bump = whitelist.bump,
        has_one = authority,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub authority: Signer<'info>,
    pub mint: Account<'info, Mint>,
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(
        seeds = [b"whitelist", whitelist.mint.as_ref()],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.mint == whitelist.mint,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Whitelist {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub bump: u8,
    pub entries: Vec<Pubkey>,
}

#[error]
pub enum WhitelistError {
    #[msg("Address not whitelisted.")]
    NotWhitelisted,
}
