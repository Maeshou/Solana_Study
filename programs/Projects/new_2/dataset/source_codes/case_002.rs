use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner003IDABCDEF1234567890");

#[program]
pub mod case_003_mixed {
    use super::*;

    /// 完全ガード：Vault.owner を has_one で検証し、authority の署名を必須化
    pub fn strict_deposit(ctx: Context<StrictDepositCtx>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault_account;
        let current = **vault.to_account_info().try_borrow_lamports()?;
        **vault.to_account_info().try_borrow_mut_lamports()? = current + amount;
        msg!("strict_deposit: {} lamports", **vault.to_account_info().lamports.borrow());
        Ok(())
    }

    /// 部分ガード：authority の署名は検証するが、Vault.owner は未検証
    pub fn semi_deposit(ctx: Context<SemiDepositCtx>, amount: u64) -> Result<()> {
        let vault_info = &mut ctx.accounts.vault_blob;
        let current = **vault_info.try_borrow_lamports()?;
        **vault_info.try_borrow_mut_lamports()? = current + amount;
        msg!("semi_deposit (owner unchecked): {} lamports", **vault_info.try_borrow_lamports()?);
        Ok(())
    }

    /// 全く検証なし：raw AccountInfo で署名も所有権もチェックせず
    pub fn open_deposit(ctx: Context<OpenDepositCtx>, amount: u64) -> Result<()> {
        let vault_info = &mut ctx.accounts.vault_blob;
        let current = **vault_info.try_borrow_lamports()?;
        **vault_info.try_borrow_mut_lamports()? = current + amount;
        msg!("open_deposit (no checks): {} lamports", **vault_info.try_borrow_lamports()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StrictDepositCtx<'info> {
    /// ここで Vault.owner == authority キーを検証
    #[account(mut, has_one = authority)]
    pub vault_account: Account<'info, Vault>,
    /// 署名を必須化
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SemiDepositCtx<'info> {
    /// CHECK: raw AccountInfo、プログラム所有者チェックをスキップ
    pub vault_blob: AccountInfo<'info>,
    /// 署名は検証するが、vault_blob.owner は未検証
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OpenDepositCtx<'info> {
    /// CHECK: raw AccountInfo、所有権・署名チェックなし
    pub vault_blob: AccountInfo<'info>,
    /// CHECK: raw AccountInfo、誰でもアクセス可能
    pub user_blob: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    /// 正当なオーナーの Pubkey
    pub authority: Pubkey,
}
