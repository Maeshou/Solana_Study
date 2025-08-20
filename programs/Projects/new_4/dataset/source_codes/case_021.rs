// 8. 設定管理＋監査ログ
use anchor_lang::prelude::*;

declare_id!("ABCDEFABCDEFABCDEFABCDEFABCDEFAB");

#[program]
pub mod reinit_settings_ext_v3 {
    use super::*;

    // モードとレベルを初期化
    pub fn init_config(
        ctx: Context<InitConfig>,
        mode: u8,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.config;
        cfg.mode = mode;
        cfg.level = 1;
        Ok(())
    }

    // レベルを上げる
    pub fn upgrade_level(ctx: Context<ModifyConfig>) -> Result<()> {
        let cfg = &mut ctx.accounts.config;
        cfg.level = cfg.level + 1;
        Ok(())
    }

    // フラグを毎回書き換え
    pub fn set_flag(
        ctx: Context<ModifyConfig>,
        flag: bool,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.config;
        cfg.flag = flag;
        Ok(())
    }

    // 監査ログを毎回上書き
    pub fn write_audit(
        ctx: Context<ModifyConfig>,
        entry: String,
    ) -> Result<()> {
        let audit = &mut ctx.accounts.audit_account;
        audit.last = entry;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(init, payer = user, space = 8 + 1 + 1 + 1)]
    pub config: Account<'info, ConfigData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyConfig<'info> {
    #[account(mut)]
    pub config: Account<'info, ConfigData>,
    #[account(mut)]
    pub audit_account: Account<'info, AuditEntry>,
}

#[account]
pub struct ConfigData {
    pub mode: u8,
    pub level: u8,
    pub flag: bool,
}

#[account]
pub struct AuditEntry {
    pub last: String,
}