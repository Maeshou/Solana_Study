use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner016IDXLABLAXYZ123456");

#[program]
pub mod case_016_distinct {
    use super::*;

    /// フルガード：Vault.owner と署名を検証して直接加算
    pub fn protected_credit(ctx: Context<ProtectedCredit>, amount: u64) -> Result<()> {
        let account_info = ctx.accounts.vault_account.to_account_info();
        let before = **account_info.try_borrow_lamports()?;
        let new_total = before + amount;
        **account_info.try_borrow_mut_lamports()? = new_total;
        msg!("protected_credit 完了: {}", new_total);
        Ok(())
    }

    /// 署名のみチェック：raw AccountInfo を使って加算（owner 未検証）
    pub fn signer_credit(ctx: Context<SignerCredit>, amount: u64) -> Result<()> {
        let raw = &ctx.accounts.raw_vault;
        let prev = **raw.try_borrow_lamports()?;
        **raw.try_borrow_mut_lamports()? = prev + amount;
        msg!("signer_credit 完了: {}", **raw.try_borrow_lamports()?);
        Ok(())
    }

    /// 手動オーナーチェック：has_one は使わず if 文で比較、署名は不要
    pub fn manual_credit(ctx: Context<ManualCredit>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        let caller = ctx.accounts.user_info.key();
        if vault.owner == caller {
            let info = vault.to_account_info();
            let before = **info.try_borrow_lamports()?;
            **info.try_borrow_mut_lamports()? = before + amount;
            msg!("manual_credit 適用: {}", before + amount);
        } else {
            msg!("manual_credit: オーナーでないためスキップ");
        }
        Ok(())
    }

    /// remaining_accounts に含まれるすべてのアカウントにまとめて加算
    pub fn broadcast_credit(ctx: Context<BroadcastCredit>, amount: u64) -> Result<()> {
        for acct in ctx.remaining_accounts.iter() {
            let balance = acct.lamports();
            **acct.try_borrow_mut_lamports()? = balance + amount;
        }
        msg!("broadcast_credit 完了");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProtectedCredit<'info> {
    /// Vault.owner == authority を has_one で検証
    #[account(mut, has_one = authority)]
    pub vault_account: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignerCredit<'info> {
    /// CHECK: raw AccountInfo、所有権は検証しない
    #[account(mut)]
    pub raw_vault: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ManualCredit<'info> {
    /// mut のみ、has_one は使わず手動チェック
    #[account(mut)]
    pub vault_data: Account<'info, Vault>,
    /// CHECK: 比較のための AccountInfo
    pub user_info: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BroadcastCredit<'info> {
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意の追加アカウントを受け取る
}

#[account]
pub struct Vault {
    /// 正当なオーナーの Pubkey
    pub owner: Pubkey,
}
