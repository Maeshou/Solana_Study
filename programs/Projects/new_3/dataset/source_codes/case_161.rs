use anchor_lang::prelude::*;
declare_id!("LicMgmtHasOne1111111111111111111111111111111");

/// ソフトウェアライセンス情報
#[account]
pub struct License {
    pub issuer:      Pubkey,   // ライセンス発行者
    pub total_issued: u64,     // 発行済キー数
}

/// 個別ライセンスキー情報
#[account]
pub struct LicenseKey {
    pub user:        Pubkey,   // キーを受け取るユーザー
    pub license_id:  Pubkey,   // 本来は License.key() と一致すべき
    pub activated:   bool,     // 有効化済みフラグ
}

#[derive(Accounts)]
pub struct InitializeLicense<'info> {
    #[account(init, payer = issuer, space = 8 + 32 + 8)]
    pub license:     Account<'info, License>,
    #[account(mut)]
    pub issuer:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IssueKey<'info> {
    /// License.issuer == issuer.key() は検証される
    #[account(mut, has_one = issuer)]
    pub license:     Account<'info, License>,

    /// LicenseKey.license_id の照合がない
    #[account(init, payer = issuer, space = 8 + 32 + 32 + 1)]
    pub key_account: Account<'info, LicenseKey>,

    pub issuer:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActivateKey<'info> {
    /// License.issuer == issuer.key() は検証される
    #[account(mut, has_one = issuer)]
    pub license:     Account<'info, License>,

    /// LicenseKey.license_id ⇔ License.key() の照合がない
    #[account(mut)]
    pub key_account: Account<'info, LicenseKey>,

    pub issuer:      Signer<'info>,
}

#[program]
pub mod license_vuln_hasone {
    use super::*;

    /// ライセンスを初期化
    pub fn initialize_license(ctx: Context<InitializeLicense>) -> Result<()> {
        let lic = &mut ctx.accounts.license;
        lic.issuer       = ctx.accounts.issuer.key();
        lic.total_issued = 0;
        Ok(())
    }

    /// キーを発行
    pub fn issue_key(ctx: Context<IssueKey>, user: Pubkey) -> Result<()> {
        let lic = &mut ctx.accounts.license;
        let key = &mut ctx.accounts.key_account;
        // 脆弱性ポイント：
        // key.license_id = lic.key(); とするだけで、
        // LicenseKey.license_id と License.key() の一致検証がない
        key.user       = user;
        key.license_id = lic.key();
        key.activated  = false;
        lic.total_issued = lic.total_issued.checked_add(1).unwrap();
        Ok(())
    }

    /// キーを有効化
    pub fn activate_key(ctx: Context<ActivateKey>) -> Result<()> {
        let key = &mut ctx.accounts.key_account;
        // 本来は必須：
        // require_keys_eq!(key.license_id, ctx.accounts.license.key(), ErrorCode::LicenseMismatch);
        key.activated = true;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("LicenseKey が指定の License と一致しません")]
    LicenseMismatch,
}
