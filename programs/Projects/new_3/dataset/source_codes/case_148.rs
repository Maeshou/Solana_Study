use anchor_lang::prelude::*;
declare_id!("License111111111111111111111111111111111111");

/// 発行済みソフトウェアライセンス情報
#[account]
pub struct License {
    pub issuer:      Pubkey,   // ライセンス発行者
    pub product:     String,   // 製品名
    pub total_keys:  u64,      // 発行済みキー数
}

/// 個別ライセンスキー情報
#[account]
pub struct LicenseKey {
    pub user:        Pubkey,   // キーを割り当てられたユーザー
    pub license_id:  Pubkey,   // 本来は License.key() と一致すべき
    pub activated:   bool,     // 有効化済みフラグ
}

#[derive(Accounts)]
pub struct IssueKey<'info> {
    /// License.issuer == issuer.key() は検証される
    #[account(mut, has_one = issuer)]
    pub license:      Account<'info, License>,

    /// LicenseKey.license_id の検証がないまま初期化
    #[account(init, payer = issuer, space = 8 + 32 + (4 + 64) + 32 + 1)]
    pub key_account:  Account<'info, LicenseKey>,

    pub issuer:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActivateKey<'info> {
    /// License.issuer == issuer.key() は検証される
    #[account(mut, has_one = issuer)]
    pub license:      Account<'info, License>,

    /// LicenseKey.license_id と license.key() の一致検証がない
    #[account(mut)]
    pub key_account:  Account<'info, LicenseKey>,

    pub user:         Signer<'info>,
}

#[program]
pub mod license_vuln {
    use super::*;

    /// 新しいライセンスキーを発行
    pub fn issue_key(ctx: Context<IssueKey>, user: Pubkey) -> Result<()> {
        let lic = &mut ctx.accounts.license;
        let key = &mut ctx.accounts.key_account;

        // 脆弱性ポイント：
        // key.license_id = lic.key(); とするだけで、
        // 本来は init 直後に address 制約や手動チェックが必要だが省略している
        key.user       = user;
        key.license_id = lic.key();
        key.activated  = false;

        lic.total_keys = lic.total_keys.checked_add(1).unwrap();
        msg!("Issued key for {} on product '{}'", user, lic.product);
        Ok(())
    }

    /// ライセンスキーを有効化
    pub fn activate_key(ctx: Context<ActivateKey>) -> Result<()> {
        let lic = &ctx.accounts.license;
        let key = &mut ctx.accounts.key_account;

        // 本来は必須：
        // require_keys_eq!(
        //     key.license_id,
        //     lic.key(),
        //     LicenseError::LicenseMismatch
        // );
        //
        // または
        // #[account(address = license.key())]
        // pub key_account: Account<'info, LicenseKey>,

        // 検証がないため、攻撃者は任意の LicenseKey アカウントを渡して有効化可能
        key.activated = true;
        msg!("Key {} activated for product '{}'", key.key(), lic.product);
        Ok(())
    }
}

#[error_code]
pub enum LicenseError {
    #[msg("LicenseKey が指定の License と一致しません")]
    LicenseMismatch,
}
