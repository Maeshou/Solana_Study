use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner005IDABCDEF1234567890");

#[program]
pub mod case_005_variety {
    use super::*;

    /// パターンA：インラインでラムポーツを直接更新（オーナー＆署名チェックあり）
    pub fn inline_transfer(ctx: Context<InlineTransfer>, amount: u64) -> Result<()> {
        // has_one と signer アトリビュートで検証済みの vault をそのまま操作
        **ctx.accounts.secured_vault.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("inline_transfer 完了: {}", **ctx.accounts.secured_vault.to_account_info().lamports.borrow());
        Ok(())
    }

    /// パターンB：helper 関数経由で更新（署名のみ検証、オーナーチェックなし）
    pub fn helper_transfer(ctx: Context<HelperTransfer>, amount: u64) -> Result<()> {
        adjust(&ctx.accounts.blob_vault.to_account_info(), amount)?;
        msg!("helper_transfer 実行: {}", **ctx.accounts.blob_vault.try_borrow_lamports()?);
        Ok(())
    }

    /// パターンC：クロージャで囲って更新（オーナーチェックあり、署名不要）
    pub fn closure_transfer(ctx: Context<ClosureTransfer>, amount: u64) -> Result<()> {
        (|| -> Result<()> {
            let info = ctx.accounts.owned_vault.to_account_info();
            let new_total = **info.try_borrow_lamports()? + amount;
            **info.try_borrow_mut_lamports()? = new_total;
            Ok(())
        })()?;
        msg!("closure_transfer 実行完了");
        Ok(())
    }

    /// パターンD：フィールドを分解して更新（全く検証なし）
    pub fn destruct_transfer(ctx: Context<DestructTransfer>, amount: u64) -> Result<()> {
        let DestructTransfer { free_vault, .. } = &ctx.accounts;
        **free_vault.try_borrow_mut_lamports()? += amount;
        msg!("destruct_transfer: 検証なしで {} lamports に増加", **free_vault.try_borrow_lamports()?);
        Ok(())
    }
}

/// helper 関数：生アカウント情報でラムポーツを増やす
fn adjust(account: &AccountInfo, delta: u64) -> Result<()> {
    let current = **account.try_borrow_lamports()?;
    **account.try_borrow_mut_lamports()? = current + delta;
    Ok(())
}

#[derive(Accounts)]
pub struct InlineTransfer<'info> {
    /// オーナーと署名を同時に検証
    #[account(mut, has_one = owner_key)]
    pub secured_vault: Account<'info, Vault>,
    pub owner_key: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct HelperTransfer<'info> {
    /// AccountInfo のみ（owner チェックなし）
    pub blob_vault: AccountInfo<'info>,
    pub signer_only: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClosureTransfer<'info> {
    /// オーナーチェックのみ（has_one のみ）
    #[account(mut, has_one = owner)]
    pub owned_vault: Account<'info, Vault>,
    /// owner は署名不要なので AccountInfo
    pub owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DestructTransfer<'info> {
    /// 全く検証なしの生データ
    pub free_vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    /// 正当なオーナーの Pubkey
    pub owner_key: Pubkey,
}
