// 6. ファイル保存＋監査ログ
use anchor_lang::prelude::*;
declare_id!("FILE111122223333444455556666777788");

#[program]
pub mod misinit_file_v6 {
    use super::*;

    pub fn init_file(
        ctx: Context<InitFile>,
        uri: String,
    ) -> Result<()> {
        let f = &mut ctx.accounts.file;
        require!(uri.starts_with("http"), ErrorCode6::InvalidUri);
        f.uri = uri;
        f.size = 0;
        Ok(())
    }

    pub fn update_metadata(
        ctx: Context<InitFile>,
        new_size: u64,
    ) -> Result<()> {
        let f = &mut ctx.accounts.file;
        f.size = new_size;
        Ok(())
    }

    pub fn record_audit(
        ctx: Context<InitFile>,
        action: String,
    ) -> Result<()> {
        let log = &mut ctx.accounts.audit_log;
        log.actions.push(action);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitFile<'info> {
    #[account(init, payer = user, space = 8 + (4+128) + 8)] pub file: Account<'info, FileData>,
    #[account(mut)] pub audit_log: Account<'info, AuditLog>,
    #[account(mut)] pub user: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct FileData { pub uri: String, pub size: u64 }
#[account]
pub struct AuditLog { pub actions: Vec<String> }

#[error_code]
pub enum ErrorCode6 { #[msg("URIが無効です。http で始めてください。")] InvalidUri }
