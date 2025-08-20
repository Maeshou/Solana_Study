#[program]
pub mod fixed_reset_config {
    use super::*;

    pub fn reset_config(ctx: Context<ResetConfig>, threshold: u16) -> Result<()> {
        let cfg = &mut ctx.accounts.config;
        require_keys_eq!(cfg.admin, ctx.accounts.admin.key(), Unauthorized);
        cfg.threshold = threshold;
        cfg.active = true;
        cfg.reset_count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ResetConfig<'info> {
    #[account(init_if_needed, payer = admin, space = 8 + 4 + 1 + 4 + 32)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Config {
    pub threshold: u16,
    pub active: bool,
    pub reset_count: u32,
    pub admin: Pubkey, // ← 所有者フィールド追加
}

#[error_code]
pub enum ErrorCode {
    Unauthorized,
}