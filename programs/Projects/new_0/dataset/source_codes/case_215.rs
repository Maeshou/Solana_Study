use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token, Transfer, MintTo};

declare_id!("Fg6PaFpoGXkYsidMpW3v2u1t0s9r8q7p6o5n4m3l2k1j");

#[program]
pub mod token_vesting {
    use super::*;

    /// ベストタイムロックアカウントの初期化
    pub fn initialize_vesting(
        ctx: Context<InitializeVesting>,
        total_amount: u64,
        release_timestamp: i64,
        bump: u8,
    ) -> ProgramResult {
        let vest = &mut ctx.accounts.vesting;
        vest.beneficiary = *ctx.accounts.beneficiary.key;
        vest.total_amount = total_amount;
        vest.released = 0;
        vest.release_at = release_timestamp;
        vest.bump = bump;

        // ミントからPDAのトークンアカウントへ全額ミント
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.pda_token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        mint_to(cpi_ctx, total_amount)?;
        Ok(())
    }

    /// ロック解除タイムスタンプ到達後にトークンを請求
    pub fn claim_tokens(ctx: Context<ClaimTokens>) -> ProgramResult {
        let vest = &mut ctx.accounts.vesting;
        let now = Clock::get()?.unix_timestamp;
        require!(now >= vest.release_at, ErrorCode::TooEarly);

        let to_release = vest.total_amount
            .checked_sub(vest.released)
            .ok_or(ErrorCode::Overflow)?;
        require!(to_release > 0, ErrorCode::NothingToClaim);

        // PDA -> Beneficiary トークン転送
        let seeds = &[b"vesting", vest.beneficiary.as_ref(), &[vest.bump]];
        let signer = &[&seeds[..]];
        let cpi_accounts = Transfer {
            from: ctx.accounts.pda_token_account.to_account_info(),
            to: ctx.accounts.beneficiary_account.to_account_info(),
            authority: ctx.accounts.vesting.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );
        transfer(cpi_ctx, to_release)?;

        vest.released = vest.released.checked_add(to_release).ok_or(ErrorCode::Overflow)?;
        Ok(())
    }

    /// アカウントを取り消し、残余トークンをオーナーに返却
    pub fn revoke_vesting(ctx: Context<RevokeVesting>) -> ProgramResult {
        let vest = &ctx.accounts.vesting;
        let owner = &ctx.accounts.owner;
        require!(owner.key() == &vest.beneficiary, ErrorCode::Unauthorized);

        // PDA 残高を取り出してオーナーへ返却
        let rem = ctx.accounts.pda_token_account.amount;
        if rem > 0 {
            let seeds = &[b"vesting", vest.beneficiary.as_ref(), &[vest.bump]];
            let signer = &[&seeds[..]];
            let cpi_accounts = Transfer {
                from: ctx.accounts.pda_token_account.to_account_info(),
                to: ctx.accounts.owner_account.to_account_info(),
                authority: ctx.accounts.vesting.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                signer,
            );
            transfer(cpi_ctx, rem)?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(total_amount: u64, release_timestamp: i64, bump: u8)]
pub struct InitializeVesting<'info> {
    #[account(
        init,
        seeds = [b"vesting", beneficiary.key().as_ref()],
        bump = bump,
        payer = payer,
        space = 8 + 32 + 8 + 8 + 8 + 1,
    )]
    pub vesting: Account<'info, Vesting>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: ミント権限はオフチェーンで保有
    pub mint_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        token::mint = mint,
        token::authority = vesting,
        payer = payer,
    )]
    pub pda_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(
        mut,
        seeds = [b"vesting", vesting.beneficiary.as_ref()],
        bump = vesting.bump,
        has_one = beneficiary,
    )]
    pub vesting: Account<'info, Vesting>,

    pub beneficiary: Signer<'info>,
    #[account(mut)]
    pub pda_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub beneficiary_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RevokeVesting<'info> {
    #[account(
        mut,
        seeds = [b"vesting", vesting.beneficiary.as_ref()],
        bump = vesting.bump,
        has_one = beneficiary @ ErrorCode::Unauthorized,
    )]
    pub vesting: Account<'info, Vesting>,

    pub beneficiary: Signer<'info>,
    #[account(mut)]
    pub pda_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Vesting {
    pub beneficiary: Pubkey,
    pub total_amount: u64,
    pub released: u64,
    pub release_at: i64,
    pub bump: u8,
}

#[error]
pub enum ErrorCode {
    #[msg("You are not authorized.")]
    Unauthorized,
    #[msg("Too early to claim.")]
    TooEarly,
    #[msg("Nothing to claim.")]
    NothingToClaim,
    #[msg("Insufficient funds or overflow.")]
    Overflow,
}
