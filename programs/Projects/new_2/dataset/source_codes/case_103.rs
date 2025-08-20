use anchor_lang::prelude::*;

declare_id!("VulnCfg4444444444444444444444444444444444");

#[program]
pub mod vuln_config {
    pub fn set_limit(ctx: Context<Set>, limit: u32) -> Result<()> {
        // 誰でもロード可能・更新可能
        let mut cfg = ctx.accounts.config.load_mut()?;
        cfg.limit = limit;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set<'info> {
    pub config: AccountLoader<'info, ConfigData>,
}

#[account(zero_copy)]
pub struct ConfigData {
    pub limit: u32,
}
