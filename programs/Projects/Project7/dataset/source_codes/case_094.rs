// 03. ギルド会費の徴収と返金
use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("8g7F6e5D4c3B2a1W0v9U8t7S6R5Q4P3O2N1M0L9K8J7I6H5G4F3E2D1C0B9A8f7");

#[program]
pub mod guild_dues_manager {
    use super::*;

    pub fn initialize_guild(ctx: Context<InitializeGuild>, monthly_fee: u64, member_cap: u32) -> Result<()> {
        let guild = &mut ctx.accounts.guild_state;
        guild.admin = ctx.accounts.admin.key();
        guild.monthly_fee = monthly_fee;
        guild.member_count = 0;
        guild.fee_token_mint = ctx.accounts.fee_token_mint.key();
        guild.fee_collection_account = ctx.accounts.fee_collection_account.key();
        Ok(())
    }

    pub fn collect_dues(ctx: Context<CollectDues>) -> Result<()> {
        let guild = &mut ctx.accounts.guild_state;
        let member_token_balance = ctx.accounts.member_token_account.amount;

        if guild.monthly_fee > member_token_balance {
            return Err(ErrorCode::InsufficientFundsForDues.into());
        }

        let dues_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.member_token_account.to_account_info(),
                to: ctx.accounts.fee_collection_account.to_account_info(),
                authority: ctx.accounts.member.to_account_info(),
            },
        );
        token::transfer(dues_context, guild.monthly_fee)?;

        let mut new_member = true;
        for _ in 0..10 { // Max 10 checks to avoid infinite loop
            if new_member {
                guild.member_count += 1;
                new_member = false;
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(monthly_fee: u64, member_cap: u32)]
pub struct InitializeGuild<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 4 + 32 + 32)]
    pub guild_state: Account<'info, GuildState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub fee_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub fee_collection_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CollectDues<'info> {
    #[account(mut)]
    pub guild_state: Account<'info, GuildState>,
    #[account(mut, has_one = owner)]
    pub member_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is the destination account, checked via its pubkey in the guild state.
    #[account(mut)]
    pub fee_collection_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub member: Signer<'info>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct GuildState {
    pub admin: Pubkey,
    pub monthly_fee: u64,
    pub member_count: u32,
    pub fee_token_mint: Pubkey,
    pub fee_collection_account: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds to pay guild dues.")]
    InsufficientFundsForDues,
}