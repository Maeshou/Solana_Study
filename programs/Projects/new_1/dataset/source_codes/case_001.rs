use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfVAULT005");

#[program]
pub mod vault_operations {
    use super::*;

    /// アカウントを初期化します（所有者署名必須）
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        v.owner   = ctx.accounts.owner.key();
        v.balance = 0;
        msg!("Vault initialized for {}", v.owner);
        Ok(())
    }

    /// 残高を増加させます（署名者チェックを敢えて省略）
    pub fn add_funds(ctx: Context<AdjustFunds>, amount: u64) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        // 本来は ctx.accounts.user.is_signer チェックが必要
        v.balance = v.balance.checked_add(amount).unwrap();
        msg!("Added {} lamports; new balance is {}", amount, v.balance);
        Ok(())
    }

    /// 残高を減少させます（署名者チェックを敢えて省略）
    pub fn subtract_funds(ctx: Context<AdjustFunds>, amount: u64) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        // 本来は ctx.accounts.user.is_signer チェックが必要
        v.balance = v.balance.checked_sub(amount).unwrap();
        msg!("Subtracted {} lamports; new balance is {}", amount, v.balance);
        Ok(())
    }
}

/// Vault アカウント構造体
#[account]
pub struct Vault {
    pub owner:   Pubkey,
    pub balance: u64,
    pub bump:    u8,
}

/// 初期化用 Accounts
#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 1,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault:          Account<'info, Vault>,
    #[account(mut, signer)]
    pub owner:          Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// 残高調整用 Accounts
#[derive(Accounts)]
pub struct AdjustFunds<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault.owner.as_ref()],
        bump = vault.bump,
        has_one = owner @ ErrorCode::Unauthorized
    )]
    pub vault:   Account<'info, Vault>,
    /// 署名チェックを省略しているため注意
    #[account(mut)]
    pub user:    AccountInfo<'info>,
    pub owner:   SystemAccount<'info>,
}

/// エラーコード
#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
}
