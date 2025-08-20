use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Vault11111111111111111111111111111111111");

#[program]
pub mod timelock_vault {
    use super::*;

    /// Vault（PDA）の初期化。lock_time は UNIX タイムスタンプ
    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        lock_time: i64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.lock_time = lock_time;
        vault.balance   = 0;
        vault.bump      = *ctx.bumps.get("vault").unwrap();
        Ok(())
    }

    /// 入金：authority が署名し、自分の Vault にのみ入金可能
    pub fn deposit(ctx: Context<ModifyVault>, amount: u64) -> Result<()> {
        let authority_ai = ctx.accounts.authority.to_account_info();
        let vault_ai     = ctx.accounts.vault.to_account_info();

        // (1) authority に十分な Lamports があるか
        require!(
            **authority_ai.lamports.borrow() >= amount,
            ErrorCode::InsufficientFunds
        );

        // (2) Lamports を移動
        **authority_ai.try_borrow_mut_lamports()? -= amount;
        **vault_ai.try_borrow_mut_lamports()?     += amount;

        // (3) 状態を更新
        let vault = &mut ctx.accounts.vault;
        vault.balance = vault
            .balance
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }

    /// 引き出し：lock_time を過ぎると、自動で Vault を閉じて全 Lamports を authority に返却
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let now   = ctx.accounts.clock.unix_timestamp;

        require!(
            now >= vault.lock_time,
            ErrorCode::TooEarly
        );
        // close 属性でアカウントを閉じるだけで、deposit された Lamports＋rent が authority に返却されます
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(lock_time: i64)]
pub struct InitializeVault<'info> {
    /// PDA：["vault", authority], bump付き、同一シードでは再初期化不可
    #[account(
        init,
        seeds  = [b"vault", authority.key().as_ref()],
        bump,
        payer  = authority,
        space  = 8 + 32 + 8 + 8 + 1  // discriminator + authority + lock_time + balance + bump
    )]
    pub vault:     Account<'info, VaultAccount>,

    /// 操作権限を持つユーザー（Signer）
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyVault<'info> {
    /// has_one で authority チェック、seeds+bump で PDA を固定
    #[account(
        mut,
        seeds   = [b"vault", authority.key().as_ref()],
        bump    = vault.bump,
        has_one = authority
    )]
    pub vault:     Account<'info, VaultAccount>,

    /// 実際に署名したユーザー
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// withdraw すると同時にアカウントを閉じ、Lamports を authority に返却
    #[account(
        mut,
        seeds   = [b"vault", authority.key().as_ref()],
        bump    = vault.bump,
        has_one = authority,
        close   = authority
    )]
    pub vault:     Account<'info, VaultAccount>,

    /// 実際に署名したユーザー
    pub authority: Signer<'info>,

    /// 現在の UNIX タイムスタンプ
    pub clock:     Sysvar<'info, Clock>,
}

#[account]
pub struct VaultAccount {
    /// この Vault を操作できるユーザー
    pub authority: Pubkey,
    /// 引き出し可能になる UNIX タイムスタンプ
    pub lock_time: i64,
    /// 入金合計 Lamports
    pub balance:   u64,
    /// PDA 用 bump
    pub bump:      u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("演算でオーバーフローが発生しました")]
    Overflow,
    #[msg("残高が不足しています")]
    InsufficientFunds,
    #[msg("ロック時間を過ぎていません")]
    TooEarly,
}
