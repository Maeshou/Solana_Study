use anchor_lang::prelude::*;

declare_id!("YourProgramIdHere");

#[program]
pub mod secure_account_handling {
    use super::*;

    // 安全なパターン - アカウントタイプを明示的に指定
    pub fn process_user_account(ctx: Context<ProcessUserAccount>) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        
        // Account<'info, UserAccount> により型安全性が保証される
        msg!("Processing user account with balance: {}", user_account.balance);
        user_account.balance = user_account.balance.checked_add(100).unwrap();
        Ok(())
    }

    // 管理者アカウント用の安全な処理
    pub fn process_admin_account(ctx: Context<ProcessAdminAccount>) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;
        
        msg!("Processing admin account with permissions: {}", admin_account.permissions);
        admin_account.permissions |= 0x01; // 権限フラグを追加
        Ok(())
    }
}

// アカウント構造体の定義
#[account]
pub struct UserAccount {
    pub account_type: AccountType,
    pub owner: Pubkey,
    pub balance: u64,
    pub created_at: i64,
}

#[account]
pub struct AdminAccount {
    pub account_type: AccountType,
    pub admin: Pubkey,
    pub permissions: u64,
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum AccountType {
    User,
    Admin,
}

// 安全なアカウント構造体 - ユーザーアカウント処理用
#[derive(Accounts)]
pub struct ProcessUserAccount<'info> {
    #[account(
        mut,
        has_one = owner @ ErrorCode::Unauthorized,
        constraint = user_account.account_type == AccountType::User @ ErrorCode::InvalidAccountType
    )]
    pub user_account: Account<'info, UserAccount>,
    pub owner: Signer<'info>,
}

// 安全なアカウント構造体 - 管理者アカウント処理用
#[derive(Accounts)]
pub struct ProcessAdminAccount<'info> {
    #[account(
        mut,
        has_one = admin @ ErrorCode::Unauthorized,
        constraint = admin_account.account_type == AccountType::Admin @ ErrorCode::InvalidAccountType
    )]
    pub admin_account: Account<'info, AdminAccount>,
    pub admin: Signer<'info>,
}

// エラー定義
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid account type")]
    InvalidAccountType,
    #[msg("Unauthorized access")]
    Unauthorized,
}