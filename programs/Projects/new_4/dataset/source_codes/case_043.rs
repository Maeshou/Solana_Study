// 8. 証明書＋失効ログ（Clockなし）
use anchor_lang::prelude::*;
declare_id!("CERTZZZZYYYYXXXXWWWWVVVVUUUUTTTT");

#[program]
pub mod misinit_certificate_no_clock {
    use super::*;

    pub fn create_certificate(ctx: Context<CreateCertificate>, id: u64) -> Result<()> {
        let cert = &mut ctx.accounts.certificate;
        cert.id = id;
        cert.valid = true;
        cert.uses = 0;
        Ok(())
    }

    pub fn revoke_certificate(ctx: Context<CreateCertificate>) -> Result<()> {
        let cert = &mut ctx.accounts.certificate;
        cert.valid = false;
        cert.uses = 0;
        Ok(())
    }

    pub fn log_revocation(ctx: Context<CreateCertificate>, reason: String) -> Result<()> {
        let log = &mut ctx.accounts.revocation_log;
        if log.reasons.len() >= 5 { log.reasons.remove(0); }
        log.reasons.push(reason);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCertificate<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 1 + 1)] pub certificate: Account<'info, CertificateData>,
    #[account(mut)] pub revocation_log: Account<'info, RevocationLog>,
    #[account(mut)] pub signer: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct CertificateData { pub id:u64, pub valid:bool, pub uses:u8 }
#[account]
pub struct RevocationLog { pub reasons: Vec<String> }

