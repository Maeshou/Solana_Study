use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner009IDXXXYYYZZZ111222333");

#[program]
pub mod case_009_melange {
    use super::*;

    /// 完全ガード：has_one + signer で所有権＋署名を検証
    pub fn full_entry(ctx: Context<FullEntryCtx>, amount: u64) -> Result<()> {
        let p = ctx.accounts.protected_vault.to_account_info();
        **p.try_borrow_mut_lamports()? += amount;
        msg!("full_entry 完了");
        Ok(())
    }

    /// 署名のみチェック：ヘルパー関数経由でラムポーツを更新（所有権チェックなし）
    pub fn utility_entry(ctx: Context<UtilityEntryCtx>, amount: u64) -> Result<()> {
        apply_delta(&ctx.accounts.blob_vault, amount)?;
        msg!("utility_entry 完了");
        Ok(())
    }

    /// アンパック方式：Context 構造体を分解せず vault フィールドをそのまま操作（署名不要、所有権チェックあり）
    pub fn unpack_entry(ctx: Context<UnpackEntryCtx>, amount: u64) -> Result<()> {
        let info = ctx.accounts.vault.to_account_info();
        **info.try_borrow_mut_lamports()? += amount;
        msg!("unpack_entry 完了");
        Ok(())
    }

    /// 無制限アクセス：raw AccountInfo に直接加算（誰でも実行可能）
    pub fn raw_entry(ctx: Context<RawEntryCtx>, amount: u64) -> Result<()> {
        {
            let raw = &ctx.accounts.any_vault;
            **raw.try_borrow_mut_lamports()? += amount;
        }
        msg!("raw_entry 完了");
        Ok(())
    }
}

/// ラムポーツ更新用ヘルパー
fn apply_delta(account: &AccountInfo, delta: u64) -> Result<()> {
    **account.try_borrow_mut_lamports()? += delta;
    Ok(())
}

#[derive(Accounts)]
pub struct FullEntryCtx<'info> {
    #[account(mut, has_one = owner)]
    pub protected_vault: Account<'info, Vault>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UtilityEntryCtx<'info> {
    /// CHECK: raw AccountInfo、所有権チェックなし
    pub blob_vault: AccountInfo<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnpackEntryCtx<'info> {
    #[account(mut, has_one = key)]
    pub vault: Account<'info, Vault>,
    pub key: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RawEntryCtx<'info> {
    /// CHECK: 完全未検証
    pub any_vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
}
