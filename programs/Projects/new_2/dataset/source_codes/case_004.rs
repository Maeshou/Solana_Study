use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner006IDABCDEF1234567890");

#[program]
pub mod case_006_variation {
    use super::*;

    /// 完全ガード：Vault.owner と署名を同時に検証して直接加算
    pub fn secure_draw(ctx: Context<SecureDrawCtx>, amount: u64) -> Result<()> {
        // 直接 Account 構造体から AccountInfo を取得し、インラインで加算
        **ctx.accounts.vault_acc.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("secure_draw executed");
        Ok(())
    }

    /// 署名のみチェック：クロージャを使って部分的に加算（オーナーチェックなし）
    pub fn auth_draw(ctx: Context<AuthDrawCtx>, amount: u64) -> Result<()> {
        (|| -> Result<()> {
            **ctx.accounts.partial_acc.to_account_info().try_borrow_mut_lamports()? += amount;
            Ok(())
        })()?;
        msg!("auth_draw executed");
        Ok(())
    }

    /// 無検証：ループ構文で加算（オーナー・署名チェックともになし）
    pub fn public_draw(ctx: Context<PublicDrawCtx>, amount: u64) -> Result<()> {
        for _ in 0..1 {
            **ctx.accounts.free_acc.to_account_info().try_borrow_mut_lamports()? += amount;
        }
        msg!("public_draw executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SecureDrawCtx<'info> {
    /// Vault.owner == signer_pubkey を検証
    #[account(mut, has_one = signer_pubkey)]
    pub vault_acc: Account<'info, Vault>,
    /// 署名チェック
    pub signer_pubkey: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AuthDrawCtx<'info> {
    /// CHECK: raw AccountInfo、owner チェック省略
    pub partial_acc: AccountInfo<'info>,
    /// 署名のみチェック
    pub user_sig: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PublicDrawCtx<'info> {
    /// CHECK: 完全に未検証
    pub free_acc: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    /// 正当なオーナーの Pubkey
    pub signer_pubkey: Pubkey,
}
