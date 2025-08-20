use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner033IDXREQUIREWARNPARTALL");

#[program]
pub mod case_033_diverse {
    use super::*;

    /// パターン1：require! マクロでオーナーのみ検証して入金
    pub fn deposit_require(ctx: Context<RequireCtx>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault_account;
        // オーナーキーが一致しなければエラー
        require!(
            vault.owner == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );
        **vault.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("deposit_require 完了");
        Ok(())
    }

    /// パターン2：WARN メッセージだけ出して、所有権チェックなしで入金
    pub fn deposit_warn(ctx: Context<WarnCtx>, amount: u64) -> Result<()> {
        msg!("Warning: owner check skipped");
        **ctx.accounts.raw_vault.try_borrow_mut_lamports()? += amount;
        msg!("deposit_warn 完了");
        Ok(())
    }

    /// パターン3：remaining_accounts の偶数番アカウントにだけ入金（owner 未検証）
    pub fn deposit_partition(ctx: Context<PartitionCtx>, amount: u64) -> Result<()> {
        for (idx, acct) in ctx.remaining_accounts.iter().enumerate() {
            if idx % 2 == 0 {
                **acct.try_borrow_mut_lamports()? += amount;
            }
        }
        msg!("deposit_partition 完了");
        Ok(())
    }

    /// パターン4：remaining_accounts 全件に一括入金（完全未検証）
    pub fn deposit_all(ctx: Context<AllCtx>, amount: u64) -> Result<()> {
        for acct in &ctx.remaining_accounts {
            **acct.try_borrow_mut_lamports()? += amount;
        }
        msg!("deposit_all 完了");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RequireCtx<'info> {
    #[account(mut)]
    pub vault_account: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WarnCtx<'info> {
    /// CHECK: owning program 未検証
    #[account(mut)]
    pub raw_vault: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PartitionCtx<'info> {
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意の追加アカウントを含む
}

#[derive(Accounts)]
pub struct AllCtx<'info> {
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意の追加アカウントを含む
}

#[account]
pub struct Vault {
    /// 正当なオーナーの Pubkey
    pub owner: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: caller is not the vault owner")]
    Unauthorized,
}
