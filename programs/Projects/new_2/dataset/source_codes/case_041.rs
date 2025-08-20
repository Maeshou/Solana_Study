use anchor_lang::prelude::*;
use std::ops::Add;

declare_id!("ProgMixedOwner025IDXNEWSCENE555666777");

#[program]
pub mod case_025_varied {
    use super::*;

    /// 関数A：has_one + signer で完全検証し、均一に加算
    pub fn guarded_allocate(ctx: Context<GuardedAllocate>, lamports: u64) -> Result<()> {
        let vault_info = ctx.accounts.vault_account.to_account_info();
        let previous = vault_info.lamports();
        let updated = previous.saturating_add(lamports);
        **vault_info.try_borrow_mut_lamports()? = updated;
        msg!("guarded_allocate → {} ▶ {}", previous, updated);
        Ok(())
    }

    /// 関数B：署名のみ検証、checked_add を用いて安全に更新（オーナーチェックなし）
    pub fn signer_allocate(ctx: Context<SignerAllocate>, lamports: u64) -> Result<()> {
        let raw = &ctx.accounts.raw_account;
        let current = **raw.try_borrow_lamports()?;
        let new_balance = current.checked_add(lamports).unwrap_or(current);
        **raw.try_borrow_mut_lamports()? = new_balance;
        msg!("signer_allocate ▶ {}", new_balance);
        Ok(())
    }

    /// 関数C：matches! マクロで手動オーナーチェック（has_one は使わず）
    pub fn pattern_allocate(ctx: Context<PatternAllocate>, lamports: u64) -> Result<()> {
        let vault = &ctx.accounts.data_vault;
        matches!(vault.owner == ctx.accounts.caller.key(), true).then(|| {
            let info = vault.to_account_info();
            **info.try_borrow_mut_lamports().unwrap() += lamports;
        });
        msg!("pattern_allocate 完了");
        Ok(())
    }

    /// 関数D：remaining_accounts に含まれる全アカウントへ一括適用（無検証）
    pub fn flood_allocate(ctx: Context<FloodAllocate>, lamports: u64) -> Result<()> {
        ctx.remaining_accounts.iter().for_each(|acct| {
            // 任意のアカウントが渡される想定
            let _ = acct.try_borrow_mut_lamports().map(|mut lam| *lam = lam.add(lamports));
        });
        msg!("flood_allocate 完了");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GuardedAllocate<'info> {
    /// Vault.owner == authority を has_one で検証
    #[account(mut, has_one = authority)]
    pub vault_account: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignerAllocate<'info> {
    /// CHECK: raw AccountInfo、所有権チェックなし
    #[account(mut)]
    pub raw_account: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PatternAllocate<'info> {
    /// mut のみ、has_one は使わず手動チェック
    #[account(mut)]
    pub data_vault: Account<'info, Vault>,
    /// CHECK: 比較用に AccountInfo を受け取る
    pub caller: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FloodAllocate<'info> {
    /// CHECK: remaining_accounts に任意のアカウントが渡される
    pub system_program: Program<'info, System>,
    // remaining_accounts に追加の AccountInfo が含まれる
}

#[account]
pub struct Vault {
    /// 許可されたオーナーの Pubkey
    pub owner: Pubkey,
}
