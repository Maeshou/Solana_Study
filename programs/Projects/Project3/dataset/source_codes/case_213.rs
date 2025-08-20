use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token, MintTo, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpW4z3y2x1w0v9u8t7s6r5q4o3p2n");

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
        wl.entries = Vec::new();
        wl.bump = bump;
        Ok(())
    }

    /// ホワイトリストへのアドレス追加
    pub fn add_member(
        ctx: Context<ModifyWhitelist>,
        member: Pubkey,
    ) -> ProgramResult {
        let wl = &mut ctx.accounts.whitelist;
        require!(ctx.accounts.authority.key == &wl.authority, ErrorCode::Unauthorized);
        require!(!wl.entries.contains(&member), ErrorCode::AlreadyMember);
        wl.entries.push(member);
        Ok(())
    }

    /// ホワイトリストからアドレス削除
    pub fn remove_member(
        ctx: Context<ModifyWhitelist>,
        member: Pubkey,
    ) -> ProgramResult {
        let wl = &mut ctx.accounts.whitelist;
        require!(ctx.accounts.authority.key == &wl.authority, ErrorCode::Unauthorized);
        wl.entries.retain(|&x| x != member);
        Ok(())
    }

    /// ホワイトリスト参加者によるNFTミント
    pub fn mint_nft(
        ctx: Context<MintNft>,
        bump: u8,
    ) -> ProgramResult {
        let wl = &ctx.accounts.whitelist;
        let user = &ctx.accounts.user;
        require!(wl.entries.contains(&user.key()), ErrorCode::NotWhitelisted);

        // PDAからユーザーアカウントへ1トークンミント
        let seeds = &[b"whitelist", ctx.accounts.mint.key().as_ref(), &[bump]];
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

/// 初期化とホワイトリスト変更のコンテキスト
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeWhitelist<'info> {
    #[account(
        init,
        seeds = [b"whitelist", mint.key().as_ref()],
        bump = bump,
        payer = authority,
        space = 8 + 32 + 4 + 100 * 32 + 1,
    )]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ModifyWhitelist<'info> {
    #[account(
        mut,
        seeds = [b"whitelist", mint.key().as_ref()],
        bump = whitelist.bump,
        has_one = authority @ ErrorCode::Unauthorized,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub authority: Signer<'info>,
    pub mint: Account<'info, Mint>,
}

/// ミントのコンテキスト
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct MintNft<'info> {
    #[account(
        seeds = [b"whitelist", mint.key().as_ref()],
        bump = bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.mint == mint.key(),
        error = ErrorCode::Unauthorized,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Whitelist {
    pub authority: Pubkey,
    pub entries: Vec<Pubkey>,
    pub bump: u8,
}

#[error]
pub enum ErrorCode {
    #[msg("Unauthorized.")]
    Unauthorized,
    #[msg("Already a member.")]
    AlreadyMember,
    #[msg("Not whitelisted.")]
    NotWhitelisted,
}
