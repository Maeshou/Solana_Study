use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUM");

#[program]
pub mod certificate_manager {
    use super::*;

    /// 証明書発行：メタデータと受取人を受け取り、初期フィールドだけ設定
    pub fn issue_certificate(
        ctx: Context<IssueCertificate>,
        metadata: String,
    ) -> Result<()> {
        let cert = &mut ctx.accounts.certificate;
        // 全体はゼロクリア済 ⇒ 必要なフィールドだけ代入
        cert.issuer         = ctx.accounts.issuer.key();
        cert.bump           = *ctx.bumps.get("certificate").unwrap();
        cert.cert_id        = ctx.accounts.cert_id;
        cert.recipient      = ctx.accounts.recipient.key();
        cert.metadata       = metadata;
        let now = ctx.accounts.clock.unix_timestamp;
        cert.issued_ts      = now;
        cert.last_action_ts = now;
        Ok(())
    }

    /// 証明書検証：最後の検証時刻を更新
    pub fn verify_certificate(
        ctx: Context<ModifyCertificate>,
    ) -> Result<()> {
        let cert = &mut ctx.accounts.certificate;
        cert.last_action_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// 証明書失効：is_revoked を立て、時刻を更新
    pub fn revoke_certificate(
        ctx: Context<ModifyCertificate>,
    ) -> Result<()> {
        let cert = &mut ctx.accounts.certificate;
        cert.is_revoked     = true;
        cert.last_action_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(cert_id: u64)]
pub struct IssueCertificate<'info> {
    /// ゼロクリア後、必要フィールドだけ設定
    #[account(
        init_zeroed,
        payer = issuer,
        seeds = [b"certificate", issuer.key().as_ref(), &cert_id.to_le_bytes()],
        bump,
        space = 8 + 32 + 1 + 8 + 32 + 4 + 100 + 1 + 8 + 8
    )]
    pub certificate: Account<'info, Certificate>,

    /// 証明書発行者（署名必須）
    #[account(mut)]
    pub issuer: Signer<'info>,

    /// 証明書受取人（キーだけ必要なため未チェックアカウント）
    pub recipient: UncheckedAccount<'info>,

    /// 証明書ID（PDA にも使用）
    pub cert_id: u64,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyCertificate<'info> {
    /// 既存の証明書（PDA 検証 + オーナーチェック）
    #[account(
        mut,
        seeds = [b"certificate", certificate.issuer.as_ref(), &certificate.cert_id.to_le_bytes()],
        bump = certificate.bump,
        has_one = issuer
    )]
    pub certificate: Account<'info, Certificate>,

    /// 証明書発行者（署名必須）
    #[account(signer)]
    pub issuer: AccountInfo<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Certificate {
    pub issuer:          Pubkey,  // 発行者
    pub bump:            u8,      // PDA 用バンプ
    pub cert_id:         u64,     // 証明書ID
    pub recipient:       Pubkey,  // 受取人
    pub metadata:        String,  // 証明書メタデータ
    pub is_revoked:      bool,    // 失効フラグ
    pub issued_ts:       i64,     // 発行時刻
    pub last_action_ts:  i64,     // 最終操作時刻
}
