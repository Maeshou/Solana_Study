// 8. 証明書発行＋失効ログ
use anchor_lang::prelude::*;
declare_id!("CERTAAAABBBBCCCCDDDDEEEEFFFF4444");

#[program]
pub mod misinit_certificate_v4 {
    use super::*;

    pub fn create_certificate(ctx: Context<CreateCertificate>, id: u64) -> Result<()> {
        let cert = &mut ctx.accounts.certificate;
        cert.id = id;
        cert.valid = true;
        cert.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn revoke_certificate(ctx: Context<CreateCertificate>, reason: String) -> Result<()> {
        let cert = &mut ctx.accounts.certificate;
        cert.valid = false;
        let log = &mut ctx.accounts.revocation_log;
        log.entries.push((reason, Clock::get()?.unix_timestamp));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCertificate<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 1 + 8)] pub certificate: Account<'info, CertificateData>,
    #[account(mut)] pub revocation_log: Account<'info, RevocationLog>,
    #[account(mut)] pub signer: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct CertificateData { pub id:u64, pub valid:bool, pub created_at:i64 }
#[account]
pub struct RevocationLog { pub entries: Vec<(String,i64)> }
