use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod secure_program_pattern1 {
    use super::*;

    // パターン1: 明示的なSignerチェックを使用
    pub fn transfer_tokens(
        ctx: Context<TransferTokens>,
        amount: u64,
    ) -> Result<()> {
        // 明示的にSignerチェックを実行
        if !ctx.accounts.authority.is_signer {
            return Err(ErrorCode::UnauthorizedSigner.into());
        }

        // さらに安全性を高めるため、権限の所有者も確認
        if ctx.accounts.authority.key() != ctx.accounts.user_token_account.owner {
            return Err(ErrorCode::InvalidOwner.into());
        }

        // トークン転送の実行
        let cpi_accounts = anchor_spl::token::Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.destination_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        anchor_spl::token::transfer(cpi_ctx, amount)?;

        msg!("Successfully transferred {} tokens", amount);
        Ok(())
    }

    pub fn update_user_data(
        ctx: Context<UpdateUserData>,
        new_data: String,
    ) -> Result<()> {
        // 複数のSignerチェック
        if !ctx.accounts.user.is_signer {
            return Err(ErrorCode::UnauthorizedSigner.into());
        }

        // データ所有者の確認
        if ctx.accounts.user_data.owner != ctx.accounts.user.key() {
            return Err(ErrorCode::InvalidDataOwner.into());
        }

        // データの更新
        ctx.accounts.user_data.data = new_data;
        ctx.accounts.user_data.last_updated = Clock::get()?.unix_timestamp;

        msg!("User data updated successfully");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // Signerトレイト使用で自動チェック
    
    #[account(
        mut,
        constraint = user_token_account.owner == authority.key() @ ErrorCode::InvalidTokenOwner
    )]
    pub user_token_account: Account<'info, anchor_spl::token::TokenAccount>,
    
    #[account(mut)]
    pub destination_token_account: Account<'info, anchor_spl::token::TokenAccount>,
    
    pub token_program: Program<'info, anchor_spl::token::Token>,
}

#[derive(Accounts)]
pub struct UpdateUserData<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // 自動的にSignerチェック
    
    #[account(
        mut,
        constraint = user_data.owner == user.key() @ ErrorCode::InvalidDataOwner,
        seeds = [b"user_data", user.key().as_ref()],
        bump
    )]
    pub user_data: Account<'info, UserData>,
}

#[account]
pub struct UserData {
    pub owner: Pubkey,
    pub data: String,
    pub last_updated: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized signer")]
    UnauthorizedSigner,
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Invalid token owner")]
    InvalidTokenOwner,
    #[msg("Invalid data owner")]
    InvalidDataOwner,
}